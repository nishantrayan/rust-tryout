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

  let v  = vec![1, 2, 3];
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

}

fn print_coordinates((x, y): &(i32, i32)) {
  println!("Current location: ({x}, {y})");
}
