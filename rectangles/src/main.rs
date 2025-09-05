#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.area() > rectangle.area()
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    let square = Rectangle::square(20);

    println!("square has an edge size of {}", square.width);
    println!("square is {square:#?}");
    println!("rect1 is {rect1:#?}");
    println!(
        "The method area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!(
        "The function area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));
}

fn area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}
