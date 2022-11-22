fn  main() {
    vector();
    string();
    hashmap();
}

fn hashmap() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    let team_name = "Blue";
    // get an option<ref>, create a option<value> and unwrap the option to get the copy
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score: {:?}", score);

    // Iterate
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // Note insert takes ownership of the arguments

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }
    {
        // Updating
        println!("{:?}", scores);

        // overwrite existing value
        scores.insert("Blue", 20);
        println!("{:?}", scores);

        // addd if key does not exist in map:
        scores.entry("Purple").or_insert(50);
        scores.entry("Blue").or_insert(50);
        println!("{:?}", scores);

        // get entry and assign
        //  note that `v` is not mutable, as v is an Entry object which may or may not contain a reference
        //  slightly confusion from this syntax :)
        let v = scores.entry("Purple").or_insert(1);
        *v += 1;
        println!("{:?}", v);
    }
}

fn string() {
    // str: slice stored somewhere else (a view)
    // String: container (like vector) of characters 
    {
        // Constructor
        let s = String::new();
        println!("s: {:?}", s);
    }
    {
        // str implements Display/ToString trait
        let s = "initial contents".to_string();
        println!("s: {:?}", s);
    }
    {
        let s = String::from("initial contents");
        println!("s: {:?}", s);
    }
    {
        // update: add single char:
        let mut s = String::from("lo");
        s.push('l');
        println!("s: {:?}", s);

        // update: add string
        let mut s = String::from("foo");
        s.push_str("bar");
        println!("s: {:?}", s);

        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        // interesting signature:
        //  add takes ownership of s1 to create s3
        let s3 = s1 + &s2 ;
        println!("s3: {:?}", s3);

        // alternative, which does not take ownership
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("s3: {:?}", s);
    }
    {
        // Indexing
        //  not supported on String due to UTF8 encoding a character can be bigger then a single byte
        //  you could use ranges to create a slice, but conditions apply (ranges must hold complete characters)
        // Instead: iterate!
        for c in "Зд".chars() {
            println!("{}", c);
        }
        for b in "Зд".bytes() {
            println!("{}", b);
        }
    }
}

fn vector() {
    {
        // Constructor
        let v: Vec<i32> = Vec::new();
        println!("v: {:?}", v);
    }

    {
        // macro to create and initialize
        let v = vec![1, 2, 3];
        println!("v: {:?}", v);
    }
    
    {
        let mut v = Vec::new();

        // updating
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
        println!("v: {:?}", v);
    }

    {
        let v = vec![1, 2, 3, 4, 5];
    
        // accessors
        let third: &i32 = &v[2]; // with optional type annotation
        println!("The third element is {}", third);

        let first = &v[0];
        println!("The first element is {}", first);

        let third: Option<&i32> = v.get(2); // get gives an Option<T>
        match third {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }

        // out-of-range
        // let out_of_range = &v[100]; -> would cause a panic
        let does_not_exist = v.get(100);
        match does_not_exist {
            Some(n) => println!("The does_not_exist element is {}", n),
            None => println!("There is no does_not_exist element."),
        }

        // iteration (with reference)
        for i in &v {
            println!("{}", i);
        }

        // iteration (copy)
        for i in v {
            println!("{}", i);
        }

        // iteration with mutation
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
            println!("{}", i);
        }
    }
    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
    
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
        
        for i in &row {
            match i {
                SpreadsheetCell::Int(n) => println!("Cell with int: {}", n),
                SpreadsheetCell::Float(n) => println!("Cell with float: {}", n),
                SpreadsheetCell::Text(n) => println!("Cell with text: {}", n),
            }
        }        
    }
    {
        
    }
}
