#[derive(Debug)]

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn vector() {
    let mut vec: Vec<i32> = Vec::new();
    let mut val: i32;

    for _i in 0..5 {
        println!("Enter a value: ");
        val = read!();
        vec.push(val);
    }

    //Iterating and printing a vector
    for i in vec.iter() {
        print!("{}", i);
        print!(" ");
    }

    println!();

    //Reversing a vector
    for i in vec.iter().rev() {
        print!("{}", i);
        print!(" ");
    }

    //Match in a vector
    match vec.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    //Vector of enum
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in row.iter() {
        println!("{:?}", i);
    }
}