struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    println!("Hello, world!");

    let is_bigger = 1 > 4;
    println!("Is 1 > 4? {}", is_bigger);

    // Addition, Subtraction, and Multiplication
    println!(
        "1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}",
        1u32 + 2,
        8i32 - 5,
        15 * 3
    );

    // Integer and Floating point division
    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);

    // Integer literal
    let _i = 1000; // i32 assigned by default
    let _i: i32 = 1000i32; // suffix for type direction
    let _i: i32 = 98_222; // = 98222 Decimal for visual separation
    let _i: i32 = 0xff; // Hex
    let _i: i32 = 0o77; // Octal
    let _i: i32 = 0b1111_0000; // Binary
    let _i: u8 = b'A'; // Byte (u8 only)

    // Float literal
    let _number_64 = 4.0; // compiler infers the value to use the default type f64
    let _number_32: f32 = 5.0; // type f32 specified via annotation

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    let f = five();
    println!("{}", f);
    let mut a = [1, 2, 3, 4, 5];
    a[1] = 100;

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number); // 5

    let condition = 10;

    let number = if condition > 4 {
        if condition > 8 {
            11
        } else {
            5
        }
    } else {
        3
    };

    println!("{}", number);

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);
}

fn five() -> i32 {
    return 5; // no semicolon
}
