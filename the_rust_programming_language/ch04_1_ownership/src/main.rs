
fn main() {
    let mut s = String::from("hello");
    println!("Original s: {s}");

    s.push_str(", world!");
    println!("New s: {s}");

    // move operation as we are working a type allocated with heap memory
    // and without a Copy trait, s becomes invalid
    let s2 = s;
    // println!("s: {s}");
    println!("s2: {s2}");

    // deepcopy via clone:
    let s3 = s2.clone();
    println!("s2: {s2}");
    println!("s3: {s3}");

    // Since s3 has no Copy trait, it is moved into takes_ownership
    takes_ownership(s3);

    let x = 5;
    makes_copy(x);

    let s4 = gives_ownership();
    println!("s4: {s4}");

    // here s4 is moved (thus becomes invalid)
    let s5 = takes_and_gives_back(s4);
    println!("s5: {s5}");
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    // move some_string to the caller
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
