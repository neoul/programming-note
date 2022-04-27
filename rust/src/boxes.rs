pub fn example() {
    use std::mem;

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Point {
        x: f64,
        y: f64,
    }
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    fn origin() -> Point {
        Point { x: 0f64, y: 0f64 }
    }
    fn boxed_origin() -> Box<Point> {
        // allocate a point to heap
        return Box::new(Point { x: 0.0, y: 0.0 });
    }

    // stack allocated variables
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 100.0, y: 100.0 },
    };
    // heap allocated variables
    let box_point: Box<Point> = Box::new(origin());
    let box_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 200.0, y: 200.0 },
    });

    // inner box in stack
    let double_indirect_box: Box<Box<Point>> = Box::new(boxed_origin());
    println!("{point:?}");
    println!(
        "Point occupies {} bytes on the stack",
        mem::size_of_val(&point)
    );
    println!(
        "Rectangle occupies {} bytes on the stack",
        mem::size_of_val(&rectangle)
    );

    // box size == pointer size
    println!(
        "Boxed point occupies {} bytes on the stack",
        mem::size_of_val(&box_point)
    );
    println!(
        "Boxed rectangle occupies {} bytes on the stack",
        mem::size_of_val(&box_rectangle)
    );
    println!(
        "Boxed box occupies {} bytes on the stack",
        mem::size_of_val(&double_indirect_box)
    );

    // Copy the data contained in `boxed_point` into `unboxed_point`
    let unboxed_point: Point = *box_point;
    println!(
        "Unboxed point occupies {} bytes on the stack",
        mem::size_of_val(&unboxed_point)
    );
}
