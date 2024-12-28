#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> f64 {
        (self.width * self.height).into()
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rectangle1 = Rectangle {
        width: dbg!(120 * scale),
        height: 60,
    };

    println!("The area of the rectangle is: {}", rectangle1.area());
    println!("{rectangle1:?}");
    dbg!(&rectangle1);

    let rectangle2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rectangle3 = Rectangle {
        width: 120,
        height: 65,
    };

    println!("Can rectangle1 hold rectangle 2?: {}", rectangle1.can_hold(&rectangle2));
    println!("Can rectangle1 hold rectangle 3?: {}", rectangle1.can_hold(&rectangle3));

    let rectangle4 = Rectangle::square(10);
    dbg!(&rectangle4);
}
