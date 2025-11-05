use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;
fn main() {
    let a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    println!("v = {:?}", v);

    {
        let v2 = vec![1, 2, 3];
        println!("v2 = {:?}", v2);
    }
    let mut v = vec![1, 2, 3];
    let third: &i32 = &v[2];
    v.push(6);
    for i in &mut v {
        *i += 50;
        println!("i = {i}");
    }

    for i in &v {
        println!("i = {i}");
    }

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
    match &row[1] {
        SpreadsheetCell::Int(i) => println!("Int = {i}"),
        _ => println!("Other"),
    }
    // println!("The third element is {third}");
    
    match v.get(20) {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    let s1 = String::from("Yo");
    let s2: &str = "osmething";
    let s3: String = s2.to_string();
    let s4: String = String::from("Something");

    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("s = {s}");
    let s5 = format!("{}{}", s1, s2);
    println!("s5 = {s5}");
    println!("s1 = {s1}");

    let hello = String::from("नमस्ते"); // Hindi word for Namaste
    let tamil_hello = String::from("வணக்கம்"); // Tamil word for Vanakkam
    // You can get first char using chars().nth()
    let bytes = tamil_hello.bytes();
    println!("bytes = {}", bytes.len());
    println!("bytes = {:?}", bytes);

    for c in tamil_hello.chars() {
        println!("c = {c}");
    }

    for c in tamil_hello.graphemes(true) {
        println!("c = {c}");
    }
    for g in tamil_hello.graphemes(true) {
        println!("g = {g}");
    }

    let blue = String::from("blue");
    let yellow = String::from("yellow");
    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(yellow, 50);
    let team_name = String::from("blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("key = {key}, value = {value}");
    }

    scores.insert(String::from("blue"), 25);
    println!("scores = {:?}", scores);
    scores.entry(String::from("red")).or_insert(30);
    println!("scores = {:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map = {:?}", map);
}
