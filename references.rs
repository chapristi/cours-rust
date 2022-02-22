fn main() {
    let mut  c : i32 = 10;
    //let cr  =  &mut c;
    //le fait d'ajouter 1 va l'ajouter a tout le monde
    //*cr += 1 ;
    {
        let cr  =  &mut c;

        *cr += 1 ;


    }


    println!("x = is {}",c);
}