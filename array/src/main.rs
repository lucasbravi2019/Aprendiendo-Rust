use std::io;

fn main() {
    println!("Elige el índice del array");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            println!("{}", err);
            std::panic!("Error al parsear index");
        },
    };

    let array: [i16; 5] = [1, 2, 3, 4, 5];

    println!("The number you selected is: {}", array[index]);
}
