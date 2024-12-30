pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        let result = add(2, 5);
        assert_eq!(result, 7);
    }

    #[test]
    fn can_hold() {
        let rectangle1 = Rectangle {
            width: 16,
            height: 4,
        };
        let rectangle2 = Rectangle {
            width: 15,
            height: 3,
        };

        let result = rectangle1.can_hold(&rectangle2);

        assert!(result)
    }

    #[test]
    fn cannot_hold() {
        let rectangle1 = Rectangle {
            width: 16,
            height: 4,
        };
        let rectangle2 = Rectangle {
            width: 16,
            height: 3,
        };

        let result = rectangle1.can_hold(&rectangle2);

        assert!(!result)
    }
}
