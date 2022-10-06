fn main() {
    let s = String::from("hello world");

    // slice or refernce to a setion of the string (span?)
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{hello} and {world}");

    // starting at zero means you can skip the index
    let start = &s[0..2];
    let start = &s[..2];
    println!("start: {start}");

    // same for the end
    let len = s.len();
    let end = &s[3..len];
    let end = &s[3..];
    println!("end: {end}");

    let fw = first_word(&s);
    println!("first_word: {fw}");
}

// retrun a string slice (str: immutable reference) of the string
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

