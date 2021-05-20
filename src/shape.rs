/**Making Animation is not its job that is done out side by lib and animation mod. shape just has to add it */
use crate::{Animation};//becomes visisble after r visible in lib.rs

use crate::attributes::{Attributes};

#[derive(Debug)]
pub struct Shape{
    name:String,
    no:u128,
    id:u128,
    animations:Vec<Animation>,
    attributes:Attributes,
}
//==========================================
impl Shape{
    pub fn new(n:&str)->Shape{
        let a = Attributes::new();

       Shape {
            name:String::from(n),
            no:453,
            id:45432,
            animations:Vec::new(),
            attributes:a,
        }
    }
    pub fn add_animation(&mut self,a:Animation){
        self.animations.push(a);
        }
}//end of impl block