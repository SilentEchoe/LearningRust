struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> u32 {
        self.width * self.height
    }

    fn new(w: u32, h: u32) -> Rectangle {
        Rectangle {
            width: w,
            height: h,
        }
    }
}

impl Rectangle {
    fn height(&self) -> u32 {
        self.height
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.width(),
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.height(),
    );
}
