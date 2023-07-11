fn main() {
    // Each value in Rust has an owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.
    let mut s = String::from("hello");
    /* There is a natural point at which we can return the memory our String 
    needs to the allocator: when s goes out of scope. When a variable goes out of scope, 
    Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the 
    code to return the memory. Rust calls drop automatically at the closing curly bracket.
     */
    {
        let q = String::from("hello");
        // upon leaving this scope the value owned by q is droped
    }
    s.push_str(" world");
    println!("{s}");
    takes_ownership(s);

}

fn moveit() {
    let x = 5;
    let y = x; // simple fixed size values live on the stack and are copied
    let s1 = String::from("hello");
    // "invalidate s1"
    // Rust will never automatically create “deep” copies of your data. 
    // Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.
    let mut s2 = s1;
    s2 = takes_and_gives_back(s2);
    //would:fail println!("{} world!", s1);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// taking ownership and then returning ownership with every function is a bit tedious.
// better use references.
// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
a_string  // a_string is returned and moves out to the calling function
}

fn smart_scope() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    /*
    &s1 syntax lets us create a reference that refers to the 
    value of s1 but does not own it. Because it does not own it, 
    the value it points to will not be dropped when the reference stops being used. */
    s.len()
}