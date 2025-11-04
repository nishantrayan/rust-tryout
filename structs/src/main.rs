struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let mut user1 = User {
        email: String::from("john.doe@example.com"),
        sign_in_count: 1,
        username: String::from("John Doe"),
        active: true,
    };
    let name = user1.username;
    println!("name = {name}");

    user1.username = String::from("Wallace");
    println!("user1 = {}", user1.username);

    let user2 = build_user(
        String::from("wallace2@example.com"),
        String::from("Wallace2"),
    );
    println!("user2 = {}", user2.username);

    let user3 = User {
        email: String::from("wallace3@example.com"),
        username: String::from("Wallace3"),
        ..user2
    };
    println!("user3 = {}", user3.username);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle::square(20);
    println!("rect3 = {:?} and is square = {}", rect3, rect3.is_square());
    println!("rect1 can hold rect2 = {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect2 = {}", rect2.can_hold(&rect1));
    println!("rect1 = {:#?}", rect1);
    let area = rect1.area();
    println!("area = {area}");
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}
