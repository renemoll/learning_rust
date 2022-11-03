
#[derive(Debug)] // default print/debug logic
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// multiple implementation blocks are valid
impl Rectangle {
    // assocaited function ("static")
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    {
        let width1 = 30;
        let height1 = 50;

        println!(
            "The area of the rectangle is {} square pixels.",
            area1(width1, height1)
        );
    }
    {
        let rect1 = (30, 50);

        println!(
            "The area of the rectangle is {} square pixels.",
            area2(rect1)
        );
    }

    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
    
        // println!("rect1 is {}", rect1); -> no idea how to print
        println!("rect1 is {:?}", rect1); // use Debug

        println!(
            "The area of the rectangle is {} square pixels.",
            area3(&rect1)
        );
    }
    
    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
    
        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }
    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };
    
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }
    {
        let rect1 = Rectangle::square(3);
        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }
}

// two parameters, same type, no check/warning if exchanged
fn area1(width: u32, height: u32) -> u32 {
    width * height
}

// one parameter, parameters assocaited with eachother, no idea which is which.
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
