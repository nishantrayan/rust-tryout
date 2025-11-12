// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter {}

// impl Iterator for Counter {
//     type Item = i32;
//     fn next(&mut self) -> Option<Self::Item> {
//         Some(0)
//     }
// }

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

use std::ops::Add;

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Iterator<i32> for Counter {
    fn next(&mut self) -> Option<i32> {
        Some(0)
    }
}

impl Iterator<i16> for Counter {
    fn next(&mut self) -> Option<i16> {
        Some(0)
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("Human flying");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot flying");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard flying");
    }
}

use std::fmt;
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", output);
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point2 {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point2 {}

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


struct Wrapper(Vec<String>);

use std::ops::Deref;
impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let human = Human;
    human.fly();
    Pilot::fly(&human); 
    Wizard::fly(&human);

    <Human as Wizard>::fly(&human); 

    let mut w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    let mut v = vec![String::from("hello"), String::from("world")];
    let lv = v.capacity();
    println!("lv = {:?}", lv);

    let lw = w.capacity();
    println!("lw = {:?}", lw);

    println!("v = {:?}", v);
}
