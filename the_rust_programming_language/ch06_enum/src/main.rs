
fn main() {
    {
        enum IpAddrKind {
            V4,
            V6,
        }

        #[allow(dead_code)]
        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }

        let _home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let _loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
    }
    {
        // a data field can be associated with an enum value/variant
        enum IpAddr {
            V4(String),
            V6(String),
        }
    
        let _home = IpAddr::V4(String::from("127.0.0.1"));
        let _loopback = IpAddr::V6(String::from("::1"));
    }
    {
        // data fields can be heterogeneous
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }
    
        let _home = IpAddr::V4(127, 0, 0, 1);
        let _loopback = IpAddr::V6(String::from("::1"));    
    }
    {
        #[allow(dead_code)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        // methods can be applied on enums as well.
        impl Message {
            fn call(&self) {
                // self can be any of the enum variants
            }
        }
    
        let m = Message::Write(String::from("hello"));
        m.call();
    }
    {
        #[allow(dead_code)]
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(String),
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => {
                    println!("Lucky penny!");
                    1
                }
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(s) => {
                    println!("State quarter from {:?}!", s);
                    25
                },
            }
        }

        let c = Coin::Penny;
        println!("Value: {}", value_in_cents(c));
        println!("Value: {}", value_in_cents(Coin::Quarter(String::from("there"))));
    }
    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }
    
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
        println!("Value: {:?}", six);
        println!("Value: {:?}", none);
    }
    {
        // catch-all
        #[allow(dead_code)]
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(String),
        }

        let c = Coin::Penny;
        let _v = match c {
            Coin::Penny => 1,
            Coin::Dime => 10,
            // i => i, // catch-all capture
            // alternative, if value can be ignored: '_'
            // alternative: no return/action: '_ => ()'
            _ => 0,
        };
    }
    {
        // alternative for single matches: if let
        let config_max = Some(3u8);
        match config_max {
            Some(max) => println!("The maximum is configured to be {}", max),
            _ => (),
        }

        let config_max = Some(3u8);
        if let Some(max) = config_max {
            println!("The maximum is configured to be {}", max);
        } else {
            // optional else
            println!("no match");
        }
    }
}
