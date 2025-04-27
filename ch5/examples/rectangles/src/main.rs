/**
 * An example program that calculates the area of a rectangle
 * and is refactored to use tuples and refactored again to
 * use structs
 */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods implemented by the Rectangle struct
impl Rectangle {
    // Calculates the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Determines if the rectangle can contain the second rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
