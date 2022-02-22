
struct Person{


    name : String,
    age : u8
}
pub trait ToString {
    fn to_string(&self) -> String;
}
//on appele le trait ToString
impl ToString for Person{


    fn to_string(&self) -> String {
        return format!("my name is {} and I am {}" , self.name,self.age);
    }
}

fn main(){

    let louis  = Person{name : String::from("louis"), age : 16};

    println!("{}",louis.to_string());

}

