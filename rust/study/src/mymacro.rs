pub fn introduce() {
    println!("My macro modules in study crates!");
}


#[macro_export] // macro export 필요
macro_rules! hello {
    () => {
        println!("HELLO MACRO!");
    };
}

// The arguments of a macro are prefixed by a dollar sign $ and 
// type annotated with a designator
// These are some of the available designators:
// - block
// - expr is used for expressions
// - ident is used for variable/function names
// - item
// - literal is used for literal constants
// - pat (pattern)
// - path
// - stmt (statement)
// - tt (token tree)
// - ty (type)
// - vis (visibility qualifier)

#[macro_export]
macro_rules! create_fn {
    ($fname:ident) => {
        fn $fname() {
            // stringify macro coverts an `ident` into a string
            println!("{:?}()", stringify!($fname));
        }
    };
}

#[macro_export]
macro_rules! execut_expr {
    ($expr:expr) => {
        println!("{:?}={:?}", stringify!($expr), $expr);
    };
}

// `overload!` will compare `$left` and `$right`
// in different ways depending on how you invoke it:
#[macro_export]
macro_rules! overload {
    // Arguments don't need to be separated by a comma.
    // Any template can be used!
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    };
    // ^ each arm must end with a semicolon.
    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    };
    // study::overload!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    // study::overload!(true; or false);
}

#[macro_export]
macro_rules! find_min {
    // () => ($crate::print!("\n"));
    // ($($arg:tt)*) => ({
    //     $crate::io::_print($crate::format_args_nl!($($arg)*));
    // })
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, $crate::find_min!($($y),+))
    );
}

