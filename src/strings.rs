pub fn string() {

    println!("Enter a String: ");
    //Reading a String
    let mut s : String = read!();

    println!("Enter string to be added: ");
    //Reading a substring
    let sub_string : String = read!();

    s.push(' ');

    //Pushing substring into original string
    s.push_str(&sub_string);

    println!("{}", s);
    println!();

    //Converting &str to String
    let data = "Initial Data";
    let s = data.to_string();

    println!("{} {}", data, s);

    //Concatenating using format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s4);

    //Iterating over the string in chars
    for i in s4.chars() {
        print!("{}", i);
        print!(" ");
    }

    println!();

    //Iterating over the string in bytes
    for i in s4.bytes() {
        print!("{}", i);
        print!(" ");
    }
}