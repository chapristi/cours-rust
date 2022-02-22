use std::collections::HashMap;
//hash_map est une sorte de tableau avec une key et une value
fn main() {
    let mut marks: HashMap<&str,&str> = HashMap::new();


    marks.insert("rust Program", "rust");
    marks.insert("php Program","php");
    println!("{}",marks.len());


    match marks.get("php Program"){
        Some(mark) => println!("{}",mark),
        None => println!(" you did not study this "),

    }
    marks.remove("rust Program");
    for (subject,name) in  &marks{
        println!("For {} name is {}%!", subject,name);
    }

    println!("study rust ? {}", marks.contains_key("rust Program"));



}