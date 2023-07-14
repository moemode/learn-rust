use std::collections::HashMap;

#[derive(Debug)]
enum Cell{
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {
    let v = vec![1, 2, 3];
    let third : Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }
    let mut q = vec![1, 2, 3, 4, 5];
    for i in &mut q {
        *i += 50;
        println!("{i}");
        // triggers error
        // q.push(10);
    }
    let first = &q[0];
    q.push(6);
    //println!("The first element is: {first}");
    let row = vec![
        Cell::Int(3),
        Cell::Text(String::from("Hi")),
        Cell::Float(3.14),
    ];
    dbg!(row);
    //string();
    map();
}

fn string() {
    let data = "initial contents";
    let mut s = data.to_string();
    s.push_str(" and some more.");
    let s1 = String::from("Hello ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }
}

fn map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team = String::from("Blue");
    scores.get(&team).copied().unwrap_or(0);
    for (k,v) in &scores {
        println!("{k}: {v}");
    }
    // overwrite
    scores.insert(String::from("Blue"), 20);
    // adding k-v pair if key not present
    scores.entry(String::from("Yellow")).or_insert(0);
    scores.entry(String::from("Green")).or_insert(100);
    println!("{:?}", scores);

    let mut map = HashMap::new();
    for word in "what the actual meaning is".split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
