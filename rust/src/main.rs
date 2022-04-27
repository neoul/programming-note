use study::{iterators, supertraits};

extern crate study;


fn main() {
    println!("hello rust!");
    crate::study::macros::example();

    use study::generics::Point;
    let p = Point::new(2.0, 3.0);
    println!("p.x={}, p.y={}", p.x(), p.y());

    use study::functions;
    if functions::example() == () {
        println!("function return () unit type");
    }

    crate::study::boxes::example();
    study::closures::example();
    study::traits::example();
    iterators::example();
    supertraits::example();
}
