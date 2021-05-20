
#[derive(Debug)]
pub struct Attributes {

name:String,
width:u16,
height:u16,


}

impl Attributes {
    pub fn new()->Attributes{
        Attributes {
            name: "some name".to_string(),
            width : 44,
            height : 99,
        }
    }
    pub fn set(){
        println!("set");
    }
    
}