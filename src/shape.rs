// mod super::;
// use super::Animation;
use crate::Animation;
#[derive(Debug)]
pub struct Shape{
    pub name:String,
    pub no:u128,
    pub id:u128,
    pub animations:Vec<Animation>,
}
//==========================================
impl Shape{
    pub fn new(n:&str)->Shape{
        Shape {
            name:String::from(n),
            no:453,
            id:45432,
            animations:Vec::new(),
        }
    }
}//end of impl block