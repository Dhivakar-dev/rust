fn main() {
    // let mut x = 5;
    // let x = 5;
    // println!("The value of x is: {}", x);

    // let x = 6;  // shadowing: a new variable with the same 
    // println!("The value of x is: {}", x);

    // let x = "shadowed again";  // shadowing with a different type
    // println!("The value of x is: {}", x);

    // const MAX_POINTS: u32 = 100_000;
    // println!("The maximum points are: {}", MAX_POINTS);

    // //Scalar Types
    // let x: i32 = 42; // 32-bit signed integer
    // let y: f64 = 3.14; // 64-bit floating point
    // let is_active: bool = true; // boolean
    // let letter: char = 'A'; // character

    // // Integer Literals in Rust
    // let decimal = 98_222; // Decimal
    // let hex = 0xff; // Hexadecimal
    // let octal = 0o77; // Octal
    // let binary = 0b1111_0000; // Binary
    // let byte: u8 = b'A'; // Byte (u8 only)  

    // //overflow example
    // let a: u8 = 255;
    // // let b = a + 1; // This will cause a compile-time error in debug mode  

    // //floating-point precision/types
    // let float_num: f32 = 1.23456789; // 32-bit
    // let double_num: f64 = 1.2345678901234567; // 64-bit      

    // //Numeric Operations
    // let sum = 5 + 10; // addition
    // let difference = 95.5 - 4.3; // subtraction   
    // let product = 4 * 30; // multiplication
    // let quotient = 56.7 / 32.2; // division
    // let remainder = 43 % 5; // modulus

    // //Boolean Type
    // let t: bool = true;
    // let f: bool = false;

    // //Character Type
    // let c: char = 'z';    

    // //Compound Types
    // //Tuple
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // let five_hundred = tup.0;
    // let six_point_four = tup.1;
    // let one = tup.2;

    // //Array
    // let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // let arr2 = [3; 5]; // all elements are 3
    // let first = arr[0];
    // let second = arr[1];  

    // // Calling the add function
    // let result = add(10, 20);
    // println!("The sum is: {}", result);  

    // // Calling the multiply function    
    // let product = multiply(10, 20);
    // println!("The product is: {}", product);      

    // let div = division(30, 5);
    // println!("a divided by b is: {}", div) 

    // //control flow in rust
    // let number = 6; 
    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }

    // loops in rust

    // let mut count = 0;
    // let last = loop{
    //     count += 1;
    //     println!("count is: {}", count);
    //     if count == 5 {
    //         break count;
    //     }
    // };
    // println!("The last value of count is: {}", last);

    // //while loop
    // let mut number = 3;
    // while number != 0 {
    //     println!("{}!", number);
    //     number -= 1;
    // };

    // // for loop
    // let a = [10, 20, 30, 40, 50];
    // for element in a.iter() {
    //     println!("the value is: {}", element);  
    // };

    // for loop with range
    for number in (1..4).rev() {
        println!("{}!", number);
    }


}

// //functions in rust
// fn add(a: i32, b: i32) -> i32 {
//     a + b // implicit return
// }

// //function with retrurn statement
// fn multiply(a: i32, b: i32) -> i32 {
//     return a * b; // explicit return
// }


// fn division(a: i32, b: i32) -> i32{
//     return a/b;
// }
