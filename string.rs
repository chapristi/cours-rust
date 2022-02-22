//En résumé, utilisez -le Stringsi vous avez besoin de données de chaîne détenues
//(comme transmettre des chaînes à d'autres threads ou les créer au moment de l'exécution) et utilisez -
//le &strsi vous n'avez besoin que d'une vue d'une chaîne.
fn main(){


    let hello_world : &str = "Hello, World!";
    // Nous pouvons également spécifier explicitement hello_worldla durée de vie de :
    let hello_world: &'static str = "Hello, world!";

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

