/**Making Animation is not its job that is done out side by lib and animation mod. shape just has to add it */
use crate::{Animation};//becomes visisble after r visible in lib.rs

use crate::attributes::{Attributes};

#[derive(Debug)]
pub struct Shape{
           name:String,
           animations:Vec<Animation>,
    pub    attributes:Attributes,
}
//==========================================
impl Shape{
    pub fn new(n:&str)->Shape{
        // let mut a = Attributes::new();
        //     a.set_bounding_rectangle_color("pinkish borwn".to_string());
        // println!("if i see this then its a big success {}",a.get_bounding_rectangle_color());    
       Shape {
            name:String::from(n),
            animations:Vec::new(),
            attributes:Attributes::new(),
        }
    }
    pub fn add_animation(&mut self,a:Animation){
        self.animations.push(a);
    }
    
}//end of impl block