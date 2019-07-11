use std::collections::HashMap;

pub fn hash_map() {

    //Declaration
    let mut hmap = HashMap::new();

    //Populating HashMap
    for i in 1..6 {
        println!("Enter a Name: ");
        let val : String = read!();
        hmap.insert(val, i);
    }

    //Printing
    for (key, value) in &hmap{
        println!("{} : {}", key, value);
    }

    //Update if no key
    hmap.entry(String::from("Nick")).or_insert(50);

    for(key, value) in &hmap {
        println!("{} {}", key, value);
    }

    //COUNT OF WORDS PROGRAM

    println!("Enter a String: ");
    let text : String = read!("{}\n");

    let mut new_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = new_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", new_map);

    /*OUTPUT OF ABOVE PROGRAM

    Enter a String:
    Hello Hello Hello World World World hello world
    {"World": 3, "hello": 1, "world": 1, "Hello": 3}
    */
}