// enum IpAddrKind {
//     v4(u8, u8, u8, u8),
//     // v4(String),
//     v6(String),
// }

// //message enum

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// // impl Message {
// //     fn call(&self) {
// //         // method body would be defined here
// //         println!("Message called!");
// //     }
// // }

// struct IpAddr{

//     Kind: IpAddrKind,
//     address: String,

// }

fn main() {

    // let four = IpAddrKind::v4;
    // let six = IpAddrKind::v6;

    // let localhost = IpAddr {
    //     Kind: IpAddrKind::v4,
    //     address: String::from("127.0.0.1")
    // };
    
    // let localhost = IpAddrKind::v4(127, 0, 0, 1);

    //option enum
    // enum Option<T> {
    //     Some(T),
    //     None,   
    // }

    // //option example
    // let some_number: Option<i32> = Some(5);
    // let some_string: Option<&str> = Some("a string");   
    // let absent_number: Option<i32> = None;  
    

    // //unwrap example
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y.unwrap_or(0 );
    // println!("The sum is: {}", sum);

    // value_in_cents(Coin::Quarter(UsState::Alabama));

    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    // //some_value match example
    // let some_u8_value = 0u8;
    // match some_u8_value {
    //     1 => println!("one"),
    //     3 => println!("three"),
    //     5 => println!("five"),
    //     7 => println!("seven"),
    //     _ => (), // do nothing      
    // }

    let some_value = Some(3);
    match some_value {
        Some(3) => println!("three"),
        _ => (), // do nothing  
    }

    if let Some(3) = some_value {
        println!("three");
    }
}


// fn route(ip_kind: IpAddrKind) {}


// #[derive(Debug)] // so we can inspect the state in a minute
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }

// coin enum
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }