
pub fn is_divisible_by(lhs:u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false; // explicit return
    }
    lhs % rhs == 0 // implicit return; no `;` and `return` keyword
}

// Functions that "don't" return a value, actually return the unit type `()`
pub fn example() {
    let ok = is_divisible_by(10, 2);
    println!("is divisible by {}", ok);
}
