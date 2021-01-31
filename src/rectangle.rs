#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { self.height * self.width }
    fn square(&self) -> u32 { area(self) * area(self) }
    fn can_hold(&self, rectangle2: &Rectangle) -> bool {
        self.width >= rectangle2.width &&
            self.height >= rectangle2.height
    }
    fn create_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}


pub fn main() {
    let rectangle = Rectangle {
        width: 12,
        height: 24,
    };
    println!("The area is {}", area(&rectangle));
    println!("rectangle is {:#?}", rectangle);
    println!("The area is {}", rectangle.area());
    println!("The square of area is {}", rectangle.square());

    let rectangle2 = Rectangle {
        width: 10,
        height: 24,
    };
    println!("{}", rectangle.can_hold(&rectangle2));
    println!("{}", rectangle2.can_hold(&rectangle));
    println!("{}", Rectangle::create_square(4).area())
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
