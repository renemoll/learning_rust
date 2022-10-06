// use std::intrinsics;

fn main() {
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // constant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // shadowing
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // wrapping_*: wraps the operation to the given datatype
    let a = u8::wrapping_mul(5, 64);
    println!("wrapping_mul: {a}");

    // checked_*: returns either the result or None
    match u8::checked_sub(5, 6) {
        None => println!("checked_sub: None"),
        Some(b) => println!("checked_sub: {b}")
    };

    // overflowing_: returns the wrapped value and boolean if wrapping ocoured
    let (b,c) = u8::overflowing_mul(5, 75);
    println!("overflowing_mul: {b} - wrapped: {c}");

    // saturating_: saturates the operation result
    let d = u8::saturating_add(100, 200);
    println!("saturating_add: {d}");

    // floats
    let div = 10.1 / 4.5;
    println!("division: {div}");

    // tupples and pattern matching :)
    let tup: (i32, f64, bool) = (500, 6.4, false);
    let (x, y, z) = tup;
    println!("The value of tup is: {x} {y} {z}");
    let s = tup.1;
    println!("second element: {s}");

    // arrays
    let k = [1, 2, 3, 4, 5];
    let el = k[1];
    println!("array: {el}");
    let zeros = [0;3];
    let el = zeros[2];
    println!("array: {el}");
}
