fn main() {
    let numbers : i32 = 54;
    //simmilaire au switch case
    match numbers {

        1 => println!("its one"),
        10 | 11 => println!("10 ou 11"),
        25...78 =>  println!("entre 25 et 78"),
        _ => println!("never match")

    }
}