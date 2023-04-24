#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rect {
        height: 30,
        width: 50,
    };

    dbg!(&rect1);

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("rect1 has a width");
    }

    let rect2 = Rect {
        height: 10,
        width: 40,
    };
    let rect3 = Rect {
        height: 60,
        width: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq1 = Rect::square(3);
    println!("sq1 is {:#?}", sq1);
}

fn area(rect: &Rect) -> u32 {
    rect.width * rect.height
}
