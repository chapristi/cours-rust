use std::io;
fn main() {
    let mut input = String::new();
    println!("hey say something...");
    //il faut mettre mut car input est changeable
    match io::stdin().read_line(&mut input) {
        Ok(_) => {println!("Succeeded! {} ", input);},
        Err(e) => {println!("Oops! Something went wrong!",e)}
    }
}