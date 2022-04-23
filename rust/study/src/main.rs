extern crate study;


fn main() {
    println!("hello rust!");
    study::mymacro::introduce();
    study::hello!();
    study::create_fn!(my_create_fn);
    study::execut_expr!(
        10u32 + 100u32
    );
    study::execut_expr!(
        {
            let x = 1000u32;
            let x = x * x + 2 * x - 1;
            x
        }
    );
    study::overload!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    study::overload!(true; or false);
    
    println!("{}", study::find_min!(1));
    println!("{}", study::find_min!(1 + 2, 2));
    println!("{}", study::find_min!(5, 2 * 3, 4));

    use study::generics::Point;
    let p = Point::new(2.0, 3.0);
    println!("p.x={}, p.y={}", p.x(), p.y());
}
