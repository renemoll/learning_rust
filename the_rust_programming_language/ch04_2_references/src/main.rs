
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("{s2}");

    {
        let r1 = &mut s2;
        // let r2 = &mut s; // <- only 1 mutable reference to s2 allowed at anytime.
        // let r3 = &s2;    // <- also no inmutable references if a mutalbe reference is in scope
        println!("{r1}");
    }
    {
        let r3 = &s2;
        let r4 = &s2; // multiple inmutable references are allowed
        println!("{r3} and {r4}");
    }
}

// use a reference, no transfer of ownership
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
