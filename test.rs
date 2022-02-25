

struct Rectangle {

    width : u8,
    height : u8
}

impl Rectangle {

    fn is_square(&self) -> bool {

        self.width == self.height
    }
}
fn main() {



}

fn give_two()->i32{

    return 17
}

#[cfg(test)]
//#[cfg(test)] indique a rust de compiler ce code que au moment de faire un cargo test
mod dcode_tests{
    //use super::*; on peut utiliser cette ligne pour importer tout les fonction en dehors du module de test
    #[test]
    #[should_panic]
    fn test_basic(){

        assert!(1==1);//ok!!
        panic!("Oh no!!!!");
    }
    #[test]
    #[ignore]
    fn test_equals(){
        assert_eq!(super::give_two(),17);
        assert_eq!(2,1+1);
        assert_ne!(2,1+2);
    }
    #[test]

    fn test_structs()
    {
        //le mot super permet dacceder a des fonctions... en dehors du module

        let r = super::Rectangle{

            width : 50,
            height : 50
        };
        assert!(r.is_square());
    }
}