fn main() {
    {
        let s: String = String::from("hello");
        println!("s = {s}");
    }
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}");
    takes_ownership(s1);
    // println!("s1 = {s1}");
    makes_copy(x);
    println!("x = {x}");

    let s = gives_ownership();
    println!("s = {s}");

    let s = takes_and_gives_back(s);
    println!("s1 = {s}");

    let mut s1 = String::from("hello");
    let length = calculate_length(&s1);
    println!("s1 = {s1}, length = {length}");

    change_string(&mut s1);
    println!("changed s1 = {s1}");

    let mut s2 = String::from("hello");
    let r1 = &s2;
    let r2 = &s2;
    println!("r1 = {r1}");
    let r3: &mut String = &mut s2;
    println!("r3 = {r3}");

    let mut str1: &str = "hello word";
    let word = first_word(&str1);
    println!("first_word = {word}");
}

fn first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
fn takes_ownership(some_string: String) {
    println!("some_string = {some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("some_integer = {some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    let length = deep_reference(s);
    println!("Original = {s}");
    length
}

fn deep_reference(s: &String) -> usize {
    println!("s = {s}");
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str(", world");
}
