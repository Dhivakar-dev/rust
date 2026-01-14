// struct User {
//     name: String,
//     email: String,
//     username: String,
//     sign_in_count: u64,
//     active: bool,
// }

// rectangel struct with width and height

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//area method by implementing Rectangle struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // method to check if rectangle can hold another rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height  
    }
}

//impl for associated function
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // let mut user1 = User {
    //     name: String::from("Alice"),
    //     email: String::from("alice@example.com"),
    //     username: String::from("rusty"),
    //     sign_in_count: 1,
    //     active: true,
    // }; 

    // let name = user1.name; 
    // user1.username = String::from("scorpion104");


    // let user2 = build_user(
    //     String::from("Bob"), 
    //     String::from("bob@example.com"), 
    //     String::from("bob123")
    // );

    // let user3 = User {
    //     email: String::from("james@gmail.com"),
    //     ..user2
    // };

    //tuple structs - no named fields, useful for creating different types, even if they have the same types
    // struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32);

    // let rect_dimensions = (30, 50);
    // println!("The area of the rectangle is {} square pixels.", area(rect_dimensions));

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // two rectangles to test can_hold method
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };  
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let rect4 = Rectangle::square(20);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("rect1 is {:#?}", rect1);
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("rect4 is {:#?}", rect4);
    println!("The area of the square is {} square pixels.", rect4.area());
}

// fn build_user(name: String, email: String, username: String) -> User {
//     User {
//         name,
//         email,
//         username,
//         sign_in_count: 1,
//         active: true,
//     }
// }

// // fn to calulate area of rectangle using tuple struct dimensions
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }   


// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }