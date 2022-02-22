

fn main(){
//affiche 100000 fois le nombre 2
    let numbers  = [2; 100000 ];
    let array  : [i32;5] = [1,2,5,6,2];

    for n in 0..numbers.len(){
        println!("{}",numbers[n]);
    }

    let a = [0, 1, 2, 3, 4];
    let middle = &a[1..4]; // A slice of a: just the elements 1, 2, and 3

    for e in middle.iter() {
        println!("{}", e); // Prints 1, 2, 3
    }

}

