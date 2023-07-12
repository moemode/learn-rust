//each instance of this struct owns all of its data
// possible for structs to store references to data owned by something else, but to do so requires the use of lifetime
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self{width: size, height: size}
    }
}

struct Color(i32, i32, i32);
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle::square(60);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    /*
    let u = User {
        active: true,
        username: String::from("king"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };
    let u2 = User {
        username: String::from("kingkong"),
        ..u
    };
    // u.username is still valid, u.email is not
    println!("Hello, {0}!", u.username);
    println!("Hello, {0}!", u2.username);
     */
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn rectangle_area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

fn tuple_area(dims: (u32, u32)) -> u32 {
    return dims.0 * dims.1;
}
