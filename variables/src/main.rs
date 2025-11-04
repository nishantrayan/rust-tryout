fn main() {
    let mut x = 5;
    println!("x is {x}");
    x = 6;
    println!("x is {x}");
    const SUBSCRIBER_COUNT: u32 = 100_000;
    println!("Subscriber count is {SUBSCRIBER_COUNT}");

    // In Rust, scalar data types represent a single value. The four primary scalar types are:

    // 1. Integers
    let a: i32 = -42;
    let b: u64 = 10_000;

    // 2. Floating-point numbers
    let c: f32 = 3.14;
    let d: f64 = 2.7182818284;

    let bin: i32 = 0b101010;
    let oct: i32 = 0o123;
    let hex: i32 = 0x1A;
    let byte: u8 = b'A';
    println!("bin = {bin}, oct = {oct}, hex = {hex}, byte = {byte}");

    // 3. Booleans
    let is_active: bool = true;
    let is_ready: bool = false;

    // 4. Characters
    let letter: char = 'R';
    let emoji: char = '😎';

    println!("a = {a}, b = {b}");
    println!("c = {c}, d = {d}");
    println!("is_active = {is_active}, is_ready = {is_ready}");
    println!("letter = {letter}, emoji = {emoji}");

    let tuple = ("Lets do this thing", 1);
    println!("tuple = {:?}", tuple);
    let (a, b) = tuple;
    println!("a = {a}, b = {b}");
    let dummy = [0; 10];
    println!("dummy = {:?}", dummy);
    my_function(1, 2);
    println!("result = {}", my_function(1, 2));

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number = {number}");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("result = {result}");

    counter = 3;
    while counter > 0 {
        println!("counter = {counter}");
        counter -= 1;
    }
    println!("LIFTOFF!");

    let arr = [1, 2, 3, 4, 5];
    for element in arr.iter() {
        println!("element = {element}");
    }
    println!("For loop done");
    for number in 1..4 {
        println!("range number = {number}");
    }
    // this is a comment
    /*
    this is a multi-line comment
    doc
    */
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("x is {x}");
    println!("y is {y}");
    x + y
}
