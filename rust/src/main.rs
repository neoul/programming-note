extern crate rust;

use log::{info, warn};

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

    let mystr = String::from("It is a String.");
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }
        s.len()
    }
    println!("first_word {}", first_word(&mystr));
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    let s = String::from("안녕 rust");
    let hello = &s[0..1];
    let world = &s[6..11];
    println!("{}", &hello);
}
