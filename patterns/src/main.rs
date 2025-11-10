#[derive(Debug)]
enum Language {
    English,
    Spanish,
    French,
    German,
    Italian,
    Portuguese,
    Russian,
    Chinese,
    Japanese,
}
fn main() {
    let language = Language::Chinese;
    match language {
        Language::English => println!("Hello, world!"),
        Language::Spanish => println!("Hola, mundo!"),
        Language::French => println!("Bonjour, monde!"),
        Language::German => println!("Hallo, Welt!"),
        Language::Italian => println!("Ciao, mondo!"),
        Language::Portuguese => println!("Olá, mundo!"),
        Language::Russian => println!("Привет, мир!"),
        Language::Chinese => println!("你好, 世界!"),
        Language::Japanese => println!("こんにちは, 世界!"),
        value => println!("Unknown language! {:#?}", value),
    };

    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    if let Some(status) = authorization_status {
        println!("Authorization status: {status}");
    } else if is_admin {
        println!("Admin user");
    } else if let Ok(group_id) = group_id {
        println!("Group ID: {group_id}");
    } else {
        println!("Unknown user");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    while let Some(top) = stack.pop() {
        println!("Top = {top}");
    }

    let v = vec![1, 2, 3];
    for (index, value) in v.iter().enumerate() {
        println!("Index = {index}, Value = {value}");
    }

    let x = 5;
    let (x, y, _) = (1, 2, 3);
    println!("x = {x}, y = {y}");

    let point = (3, 5);
    print_coordinates(&point);

    let x = 5;
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("x = {x}");
    } else {
        println!("x is None");
    }

    let x = Some(5);
    match x {
        Some(50) => println!("x is 50"),
        Some(y) => println!("Matched, x is {y}"),
        _ => println!("Default case, x is {x:#?}"),
    }

    let x = 2;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    match x {
        1..=5 => println!("one through five"),
        _ => println!("anything"),
    }

    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    println!("x = {a}, y = {b}");
    assert_eq!(a, 0);
    assert_eq!(b, 7);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));
    match msg {
        Message::Quit => println!("The quit message"),
        Message::Move { x, y } => println!("Move to ({x}, {y})"),
        Message::Write(text) => println!("Write message: {text}"),
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!("Change color to ({r}, {g}, {b})"),
    }

    fn foo(_: i32, y: i32) {
        println!("You called foo with 10");
    }

    let mut setting_value = Some(5);
    let new_setting_value = Some(6);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!(
                "Setting value changed from {} to {}",
                setting_value.unwrap(),
                new_setting_value.unwrap()
            );
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting_value = {:#?}", setting_value);

    let s = Some(String::from("Hello"));
    if let Some(_) = s {
        println!("s is a string");
    }
    println!("s = {:#?}", s);

    struct Point2 {
        x: i32,
        y: i32,
        z: i32,
    }
    let p = Point2 { x: 0, y: 0, z: 1 };
    match p {
        Point2 { x, .. } => println!("x = {x}"),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
      (first, .., last) => {
        println!("First number is {first}");
        println!("Last number is {last}");
      }
    }

    let num = Some(6);
    match num {
      Some(x) if x % 2 == 0 => println!("The number {x} is even"),
      Some(x) => println!("The number {x} is odd"),
      None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
      Some(n) if n == y => println!("x and y are the same"),
      _ => println!("x and y are different"),
    }

    enum Message2 {
      Hello { id: i32 },
    }

    let msg = Message2::Hello { id: 5 };
    match msg {
      Message2::Hello { id: id_variable@3..=7 } => println!("Hello id = {id_variable}"),
      _ => println!("Hello id is not important"),
    }
    
}

fn print_coordinates((x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}
