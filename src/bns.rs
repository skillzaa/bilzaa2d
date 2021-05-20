use std::collections::HashMap;

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


pub type AttributesHashMap = HashMap<String,Bns>; 
