
mod attribvalues;
use attribvalues::AttribValues;
#[derive(Debug)]
pub enum BnsEnum {B,N,S,}
#[derive(Debug)]
pub struct Bns {
    b:bool,
    n:u128,
    s:String,
    t:BnsEnum,
}
impl Bns {
    pub fn new(b:bool,n:u128,s:String,t:BnsEnum)->Bns{
        Bns {
        b,
        n,
        s,
        t,
        }
    }
}
#[derive(Debug)]

pub struct Attributes{
color:String,
rotate_angle:u128,
width:u128,
height:u128,
}
impl Attributes {
    pub fn new()->Self{
        Attributes {
            color: String::from("red"),
            rotate_angle:45,
            width:100,
            height:100,
        }
    }
    pub fn set(&mut self ,attrib_name:AttribValues,bns:Bns){
            match bns.t {
                BnsEnum::B => println!("its  a  bool"),
                BnsEnum::N => {
                  match attrib_name {
                    AttribValues::width => {self.width = bns.n },
                    _=>(),
                  }  
                },
                BnsEnum::S => println!("its  a  string"),
            }
    }
    
    pub fn get(){
    
    }
    pub fn list(){

    }
}


//==========================================
