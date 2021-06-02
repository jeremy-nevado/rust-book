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

    println!("Can rect hold rect2? {}", rect1.contains(&rect2));
    println!("Can rect hold rect3? {}", rect1.contains(&rect3));

    let sqr1 = Rectangle::square(20);

    println!("Square 1 is: {:?}", sqr1);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn contains(&self, rect: &Rectangle) -> bool {
        if self.height > rect.height && self.width > rect.width {
            return true
        } else {
            return false
        }
    }

    fn square(dim: u32) -> Rectangle {
        Rectangle {
            width: dim,
            height: dim,
        }
    }
}
