
fn generic_impl() {
    // generic implementation
    struct Val {
        val: f64,
    }
    
    struct GenVal<T> {
        gen_val: T,
    }
    // impl of Val
    impl Val {
        fn value(&self) -> &f64 {
            &self.val
        }
    }
    
    // impl of GenVal for a generic type `T`
    impl<T> GenVal<T> {
        fn value(&self) -> &T {
            &self.gen_val
        }
    }

    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    println!("{}, {}", x.value(), y.value());
}

fn trait_generics() {
    // Non-copyable types.
    struct Empty;
    struct Null;

    // A trait generic over `T`.
    trait DoubleDrop<T> {
        // Define a method on the caller type which takes an
        // additional single parameter `T` and does nothing with it.
        fn double_drop(self, _: T);
    }

    // Implement `DoubleDrop<T>` for any generic parameter `T` and caller `U`.
    impl<T, U> DoubleDrop<T> for U {
        // This method takes ownership of both passed arguments,
        // deallocating both.
        fn double_drop(self, _: T) {
        }
    }

    let empty = Empty;
    let null  = Null;

    // Deallocate `empty` and `null`.
    empty.double_drop(null);

    //empty;
    //null;
    // ^ TODO: Try uncommenting these lines.
}

fn multiple_bounds() {
    use::std::fmt::{Debug, Display};
    fn print<T: Debug + Display>(t: &T) {
        println!("{}, {:?}", t, t);
    }
    fn print_params<T: Debug, U: Debug>(t : &T, u: &U) {
        println!("{:?}, {:?}", t, u);
    }
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];
    print(&string);
    print_params(&array, &vec);
}

fn associated_types() {

}

pub fn example() {
    generic_impl();
    trait_generics();
    multiple_bounds();
    associated_types();
}

