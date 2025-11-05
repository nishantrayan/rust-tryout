#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<U> Point<U> {
    fn x(&self) -> &U {
        &self.x
    }
}

impl Point<f64> {
    fn y(&self) -> &f64 {
        &self.y
    }
}
#[derive(Debug)]
struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 { x: self.x, y: other.y }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = find_largest(number_list);

    println!("The largest number is {largest}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest = find_largest(char_list);
    println!("The largest character is {largest}");

    let point = Point { x: 1, y: 2 };
    println!("point = {:#?}", point);

    let point2 = Point { x: 1, y: 2 };
    println!("point2 = {:#?}", point2);

    let p = Point { x: 5.0, y: 10.0 };
    p.x();
    p.y();

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3 = {:#?}", p3);

}

fn find_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
