use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

#[derive(Debug)]
enum MyList {
    Cons(Rc<RefCell<i32>>, Rc<MyList>),
    Nil,
}
fn main() {
    // let value = Rc::new(RefCell::new(5));
    // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    // let b = Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::clone(&a)));
    // let c = Rc::new(Cons(Rc::new(RefCell::new(4)), Rc::clone(&a)));
    // *value.borrow_mut() += 10;
    // println!("a = {:#?}, b = {:#?}, c = {:#?}", a, b, c);


    let value= Rc::new(RefCell::new(5));
    let a = Rc::new(MyList::Cons(Rc::clone(&value), Rc::new(MyList::Nil)));
    let b = MyList::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    *value.borrow_mut() += 10;
    println!("a = {:#?}", a);
    println!("b = {:#?}", b);



}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Warning: You are using 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You are using 75% of your quota!");
        }
    }
}

mod tests {
    use super::*;
    use std::cell::RefCell;
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            let mut a = self.sent_messages.borrow_mut();
            // let mut b = self.sent_messages.borrow_mut();
            a.push(String::from(msg));
            // b.push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
