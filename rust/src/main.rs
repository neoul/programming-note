extern crate rust;


fn main() {
    println!("hello rust!");
    crate::rust::macros::example();

    use rust::functions;
    if functions::example() == () {
        println!("function return () unit type");
    }

    crate::rust::boxes::example();
    rust::closures::example();
    rust::traits::example();

    use rust::{iterators, supertraits, multipletraits};
    iterators::example();
    supertraits::example();
    multipletraits::example();
    rust::wheres::example();
    rust::generics::example();
}
