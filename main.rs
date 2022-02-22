
use std::cmp::Ordering;
use std::io;



pub struct Color{
    red : u8,
    green : u8,
    blue : u8,
    alpha: u8,
}
//permet d'ajouter des fonctions a des structures
impl Color {
    //&mut self
    fn format(&self) -> String {
        let mut result = self.red.to_string() ;
        result.push_str(self.green.to_string().as_str());
        result.push_str(self.blue.to_string().as_str());
        return result;
    }
}
fn main() {
    //mut veut seullement dire que la variable peut etre changé par la suite
    // l'underscore permet de creer des espaces entre les chiffres


    let  aint32 : i32 = 744_700_000_;
    let float  : f32 = 72.5;
    let mut string : String = String::from("bonjour");
    string.push_str(" concatenation ");

    let tab : [&str;2] = [
        "ouis",
       "Louis"
    ];
    println!("mon tableau {}",tab[0]);


    let  tuple :(i32,i64,f32,&'static str) ;
    tuple = (459, 1000000, 5.15, "sfgh");
    println!("mon tuple {:?}",tuple);

    println!("Hello, world! {}",float);
    println!("Hello, world! {}",string);
    println!("Hello, world! {}",aint32);
    println!("Hello, world! {}",aint32);




    let red  = Color{red : 255, green : 0, blue : 0, alpha : 255};



    println!("{}",red.format());


    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    let mut rng:i32 = 12;

    match guess.cmp(&rng.to_string()) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }



    println!("1 + 2 = {}", 1u32 + 2u32);

    let ys: [i32; 500] = [0; 500];

    println!("taille =  {}", ys.len());


    //let xs = [0..500];



}
