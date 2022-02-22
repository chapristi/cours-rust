
fn main(){
    //si c'est des données avec une taille fixe on peut stocker dans un array sinon vector
    //les vecteurs ne peuvent stocker que des valeurs du meme type
    //Vec<Type> on peut donc mettre ce qu'on veut nv typage
    //si on declare un vecteur comme ceci on ne peut pas lui attribuer des valeurs
    //par default
    //let mut  my_vector : Vec<i32> = Vec::new();
    //là on peut lui attribuer des valeurs par defaults c'est une macro
    let mut my_vector = vec![1, 2, 3];

    my_vector.push(5);

    {
        //Comme toutes les autres structures, un vecteur est libéré quand il sort de la portée, comme précisé dans l'encart 8-4.


        let mut my_vector = vec![1, 2, 3];
        // on fait des choses avec v

    } // <- v sort de la portée et est libéré ici

    //on peut recuperer les valeurs avec les indices comme ici ou avec la methode get
    let troisieme: &i32 = &my_vector[2];
    println!("Le troisième élément est {}", troisieme);

    match my_vector.get(2) {
        Some(troisieme) => println!("Le troisième élément est {}", troisieme),
        _ => println!("Il n'y a pas de troisième élément."),
    }

    // * va nous permttre de changer la valeur quand on choisi de regarder
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("test {} ", &v[0]);


}
