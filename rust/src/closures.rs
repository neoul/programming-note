pub fn example() {
    // Increment via closures and functions.
    fn function(i: i32) -> i32 {
        i + 1
    }

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let mut i = 1;
    i = function(i); // 2
    i = closure_annotated(i); // 3
    i = closure_inferred(i); // 4
    println!("closure example i: {}", i);
    // let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i)); // 5
    println!("closure_annotated: {}", closure_annotated(i)); // 5
    println!("closure_inferred: {}", closure_inferred(i)); // 5

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());

    use std::mem;
    let color = String::from("green");

    // bollow immutable reference of the color variable.
    let print = || println!("`color`: {}", color);
    print();

    let _reborrow = &color;
    print();

    // A move or reborrow is allowed after the final use of `print`
    let _color_moved = color;
    // print(); // 호출 불가; color already moved to _color_moved

    let mut count = 0;
    // bollow mutable count reference.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure using a mutable borrow.
    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;

    // `consume` consumes the variable so this can only be called once.
    let movable = Box::new(3);
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };
    consume();
    // consume(); // 호출 불가; The bollowed variable is not available.
    // ^ TODO: Try uncommenting this line.

    // move 사용시 ownership을 가져감
    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));

    // Closure as input parameter
    // 내부 captured variable의 type을 T로 가져와 실행 (ownership가져오므로 한번 수행)
    fn apply_t<F>(f: F) where F: FnOnce() {
        f();
    }

    // 내부 captured variable의 type을 &T로 가져와 실행
    // A function which takes a closure and returns an `i32`.
    fn apply_ref_t<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
        f(3)
    }

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };
    // Call the function which applies the closure.
    apply_t(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_ref_t(double));
}
