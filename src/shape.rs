// mod super::;
// use super::Animation;
/**Making Animation is not its job that is done out side by lib and animation mod. shape just has to add it */
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
    pub fn add_animation(&mut self,a:Animation){
        // let a  = self::Animation::new();
        self.animations.push(a);
        }
}//end of impl block