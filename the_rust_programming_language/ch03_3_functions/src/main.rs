fn main() {
    print_labeled_measurement(5, 'h');

    let x = plus_one(41);
    println!("The value of x is: {x}");

    let y = mul_two(8);
    println!("The value of x is: {y}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    // note the lack of a ';' to keep the next line an expression 
    x + 1
}

fn mul_two(x: i32) -> i32 {
    // note return keyword is available
    return x * 2;
}
