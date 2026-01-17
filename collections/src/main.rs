// use std::vec;

use std::collections::HashMap;

fn main() {
    // let a = [1,2,3];
    // let mut v: Vec<i32> = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);

    // let v2 = vec![1,2,3];

    // let v = vec![1, 2, 3, 4, 5];

    // // let third: &i32 = &v[2];
    // // println!("The third element is {}", third);

    // //accessing element using get method
    // match v.get(20) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element."), 
    // }

    // // vector iteration
    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    // }

    // for i in &v {
    //     println!("{}", i);
    // }  

    // // Using enum to store multiple types in a vector
    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }
    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];

    // match &row[0] {
    //     SpreadsheetCell::Int(value) => println!("Integer: {}", value),
    //     SpreadsheetCell::Float(value) => println!("Float: {}", value),
    //     SpreadsheetCell::Text(value) => println!("Text: {}", value),
    // } 

    // let s1 = String::new();
    // let s2 = "initial contents";
    // let s3 = s2.to_string();
    // let s4 = String::from("initial contents");

    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שלום");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    // let hello = String::from("Hola");

    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // s1.push('!');
    // println!("s1 is {s1}");

    // //appending using + operator
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2; // note s1 has been moved
    // println!("s3 is {}", s3);
    // // format! macro to concatenate strings without taking ownership
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // let s = format!("{}-{}-{}", s1, s2, s3);
    // println!("s is {}", s);

    // //string indexing
    // let s1 = String::from("hello");
    // let h = &s1[0..1];
    // println!("h is {}", h);

    // let blue = String::from("blue");
    // let yellow = String::from("yellow");

    // let mut scores = HashMap::new();

    // scores.insert(blue, 10);
    // scores.insert(yellow, 50);

    // let team_name = String::from("blue");
    // let score = scores.get(&team_name);
    // match score {
    //     Some(s) => println!("Score for {} is {}", team_name, s),
    //     None => println!("No score found for {}", team_name),   
    // }

    // // iterating over hashmap
    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }

    // // updating hashmap values

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);      

    // scores.entry(String::from("Blue")).or_insert(25);
    // scores.entry(String::from("Green")).or_insert(30);

    // udating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();   
    // ["hello", "world", "wonderful", "world"]
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);  



}
