// An integer division that doesn't `panic!`
fn checked_division(divident: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // Failure is represented as the `None` variant
        None
    } else {
        Some(divident / divisor)
    }
}

// This function handles a division that may not succeed
fn try_division(divident: i32, divisor: i32) {
    // `Option` values can be pattern matched, just like other enums
    match checked_division(divident, divisor) {
        None => println!("{} / {} failed!", divident, divisor),
        Some(quotient) => println!("{} / {} = {}", divident, divisor, quotient)
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    // Binding `None` to a variable needs to be type annotated
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_float = Some(0f32);

    // The `unwrap` method will extract the value wrapped in a `Some` variant, or will `panic!` if
    // called on a `None` variant
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());
    println!("{:?} unwraps to {:?}", none, none.unwrap());
}
