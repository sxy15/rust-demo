#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self == self: &Self
    fn area(&self) -> u32 {
         self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self{
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2: Rectangle = Rectangle {
        width: 10,
        height: 40,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    println!("square: {:?}", Rectangle::square(3));
}

