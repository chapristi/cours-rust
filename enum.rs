enum Days{

    Monday,Tuesday,Wednesday,Thursday,Friday,Saturday,Sunday
}
impl Days{

    fn is_weekday(&self) -> bool{
        return match self {
            &Days::Saturday | &Days::Sunday => false,
            //_ signifie pas que ca n'a pas d'importance donc c'est comme un else ici
            _ => true,
        }
    }
}

fn main() {

    let d = Days::Sunday;

    println!(" is d a weekday  ? {}", d.is_weekday())
}