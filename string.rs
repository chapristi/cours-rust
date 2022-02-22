
fn main(){
    let my_string = String::from("hi how are u");

    println!("Length : {} ", my_string.len());
    println!("String is empty ? {} ",my_string.is_empty());

    for token in my_string.split_whitespace(){
        println!("{} ",token);
    }

    println!("my string does contain 'hi' ? {} ",my_string.contains("hi"));
    //sert a concatener
    my_string.push_str("text lot of text");
    println!("my_string {}",my_string);


}

