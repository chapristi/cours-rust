mod dcode {
    fn chicken(){
        println!("upss!! ");
    }
    pub fn print_message(){
        chicken();
        println!("how message");
    }

    pub mod water {

        pub fn print_message(){

            println!("mod in mod lol");

        }

    }



}
fn main() {
    dcode::print_message();
    dcode::water::print_message();



}