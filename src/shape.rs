// mod super::;
// use super::Animation;
use crate::Animation;
#[derive(Debug)]
pub struct Shape{
    name:String,
    no:u128,
    id:u128,
    animations:Vec<Animation>,
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
    pub fn add_animation(&mut self){
        let a  = self::Animation::new();
        self.animations.push(a);
        }
}//end of impl block