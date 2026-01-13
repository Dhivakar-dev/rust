fn main() {
    
    //Ownership rules
    //1. Each value in Rust has a variable that’s called its owner.
    //2. There can only be one owner at a time.
    //3. When the owner goes out of scope, the value will be dropped.
    // {
    //     // string literals are stored in the stack and are immutable
    //     // let s = "hello"; // s is not the owner of the string literal
    //     // by default, variables are immutable
    //     // &str is a string slice, which is a reference to a string literal
    //     let s = String::from("hello"); // s is the owner of the String
    //     println!("{}", s);
    // } // s goes out of scope and the String is dropped here

    // // owenership transfer
    // let s1 = String::from("hello");
    // let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    // // println!("{}", s1); // this would cause a compile-time error
    // println!("{}", s2);

    // //clone method
    // let s1 = String::from("hello");
    // let s2 = s1.clone(); // s1 is cloned to s2, both are valid
    // println!("s1 = {}, s2 = {}", s1, s2);

    // //in variables and data types
    // //pimitive types are stored in the stack
    // //they implement the Copy trait
    // let x = 5; // x is an integer, which is a primitive type
    // let y = x; // x is copied to y, both are valid
    // println!("x = {}, y = {}", x, y);

    // //functions and ownership
    // let s = String::from("hello"); // s comes into scope
    // takes_ownership(s); // s's value moves into the function...
    //                           // ... and so is no longer valid here 
    // // println!("{}", s); // this would cause a compile-time error because s is no longer valid
    // let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    // println!("{}", s1);

    // let s2 = String::from("hello"); // s2 comes into scope
    // let s3 = takes_and_gives_back(s2); // s2 is moved into the function...
    //                                    // ... and then returned back to s3
    // // println!("{}", s2); // this would cause a compile-time error because s2 is no longer valid
    // println!("{}", s3);

    // //references and borrowing
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1); // we pass a reference of s1 to the function
    // println!("The length of '{}' is {}.", s1, len);

    // // mutable references
    // let mut s = String::from("hello");
    // change(&mut s);
    // println!("{}", s);

    // restrictions on mutable references
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s; // this would cause a compile-time error because we cannot have two mutable references to the same variable
    // println!("{}, {}", r1, r2);

    //rules of references
    //1. At any given time, you can have either one mutable reference or any number of immutable references.
    //2. References must always be valid.   
    //3. mutable references cannot coexist with immutable references.

    //dangling references
    // let reference_to_nothing = dangle();
// } 

// //string slice
// let s = String::from("hello world");
// let hello = &s[0..5];
// let world = &s[6..11];
// println!("{}, {}", hello, world);

// // first word function
// let s = String::from("hello world");
// let word = first_word(&s);
// println!("First word: {}", word);

// array slice
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
println!("Slice: {:?}", slice);

}

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }
// // fn that would create a dangling reference
// // fn dangle() -> &String {
// //     let s = String::from("hello");
// //     &s // we return a reference to s
// // } // s goes out of scope here and is dropped, leaving a dangling reference

// // fn takes ownership
// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// } // some_string goes out of scope and is dropped here

// // gives ownership back
// fn gives_ownership() -> String {
//     let some_string = String::from("hello");
//     some_string // some_string is returned and moves out to the caller
// }

// // takes and gives back ownership
// fn takes_and_gives_back(a_string: String) -> String {
//     a_string // a_string is returned and moves out to the caller
// }   

