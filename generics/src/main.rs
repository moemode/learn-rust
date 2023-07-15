fn main() {
    println!("Hello, world!");
}


struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T{
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
/*
largest won’t work for all possible types that T could be. 
Because we want to compare values of type T in the body,
we can only use types whose values can be ordered.
 */
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = &item;
        }
    }
    largest
}