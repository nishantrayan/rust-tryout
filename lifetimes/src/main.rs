use std::fmt::Display;
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    // println!("r = {r}");

    let x = 5;
    let r = &x;
    println!("r = {r}");

    let string1 = String::from("abcd");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    let mut result = String::new().as_str();
    {
        let string2 = String::from("xyz");
        result = first_word(string1.as_str(), string2.as_str());
    }
    println!("The first word is {result}");

    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let mut i;
    {
        // let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    println!("i = {:#?}", i);

    let mut first_word;
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        first_word = first_word_2(novel.as_str());
    }
    // println!("first_word = {first_word}");
    let string2 = String::from("xyz");
    let result = longest_with_an_announcement(string1.as_str(), string2.as_str(), "Attention please:");
    println!("The longest string is {result}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn first_word<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn first_word_2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}