
//&str est un type de slice voilà pourquoi on doit le passer en parametre
fn first_world(s : &str)-> &str{
    let bytes  = s.as_bytes();
    for (index,&byte) in bytes.iter().enumerate(){
        if byte == b' ' {
            //prend de l'index 0 a index
            return &s[index..]
        }
    }
    //renvoie tout les characteres.
    &s[..]
}
fn main() {
    let mut s : String = String::from("hello world");

    let world  = first_world(&s);
    println!("{}",world);


    let a = [1,2,3];
    let b  = &a;
    let c = &a[..2]


}