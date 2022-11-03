struct User {
    active: bool,
    username: String,   // note: the struct owns the data
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("user1: {}", user1.username);

    let user2 = build_user(
        String::from("someone@example.com"),
        String::from("someone")
    );
    println!("user2: {}", user2.username);

    let user3 = User {
        email: String::from("another@example.com"),
        ..user1 // fill unassigned fields from user1
    };
    println!("user3: {} {}", user3.username, user3.email);
    // note that user1 is not in scope anymore, as username is moved
    // println!("user1: {}", user1.username);

    // tuple structs, a tuple with a unique type
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // note black and origin are different types and hence not the same/interchangable

    // unit-like struct (i.e. tag?)
    struct AlwaysEqual;
    let a = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email,  // shorthand init assignment as parameter and field match 
        username,
        active: true,
        sign_in_count: 1,
    }
}
