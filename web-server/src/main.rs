use std::{
    error::Error,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle, Thread},
    time::Duration,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }

    fn build(size: usize) -> Result<ThreadPool, Box<dyn Error>> {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
    }
}

struct Worker {
    id: usize,
    handler: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                },
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down");
                    break;
                }
            }
        });

        Worker {
            id,
            handler: Some(thread),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(worker) = worker.handler.take() {
                worker.join().unwrap();
            }
        }
    }
}

const LOCALHOST: &str = "127.0.0.1:8080";

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 OK 200", "default_page.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "default_page.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "not_found_page.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind(LOCALHOST).unwrap();

    let thread_pool = ThreadPool::build(4).unwrap();
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        thread_pool.execute(|| handle_connection(stream));
    }

    println!("Shutting down...");
}
