pub fn example() {

    

    // where example 2
    use std::fmt::Debug;
    trait PrintInOption {
        fn print_in_option(&self);
    }
    impl<T> PrintInOption for T
    where
        T: Debug,
    {
        fn print_in_option(&self) {
            println!("{:?}", Some(self));
        }
    }
    let vec = vec![1, 2, 3];
    vec.print_in_option();
    vec.print_in_option();
    // <Vec<{i32}> as PrintInOption>::print_in_option(vec);
}
