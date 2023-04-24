struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rect {
        height: 30,
        width: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rect: &Rect) -> u32 {
    rect.width * rect.height
}
