#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated functions
    // - no self
    // - 생성자 함수 구현에 주로 사용
    fn square(size: u32) -> Rectangle {
        Rectangle{
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    // let rectx = rect;

    // dbg! return the ownership of the input object
    let rect = dbg!(rect);

    println!(
        "The area of the rectangle {:?} is {} square pixels.",
        rect,
        rect.area()
    );

    let square = Rectangle::square(100);
    let rect1 = Rectangle { height: 50, width: 30 };
    let rect2 = Rectangle { height: 40, width: 10 };
    let rect3 = Rectangle { height: 45, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can square hold rect1? {}", square.can_hold(&rect1));

}
