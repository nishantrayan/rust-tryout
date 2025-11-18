use std::fmt;

struct Age(u32);
struct ID(u32);

fn main() {

    type Kilometers = i32;
    let x: Kilometers = 5;
    let y: i32 = 10;

    println!("x = {x}, y = {y}");

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("Hello, world!"));
    f();

    // while game_in_progress() {
    //     let guess = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    // }

    let s1: &str = "hello";
    let s2: &str = "world";

}

fn generic<T: ?Sized>(t: &T) {
}

fn bar() -> ! {
    panic!("Don't return me!");
}