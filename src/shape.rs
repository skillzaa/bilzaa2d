/**Making Animation is not its job that is done out side by lib and animation mod. shape just has to add it */
use crate::{Animation};//becomes visisble after r visible in lib.rs
use uuid::Uuid;
use bilzaa2dattributes::Attributes;

#[derive(Debug)]
pub struct Shape{
           uuid:String,
           name:String,
           animations:Vec<Animation>,
    pub    attributes:Attributes,
}
//==========================================
impl Shape{
    pub fn new(name:&str)->Shape{
        let my_uuid = Uuid::new_v4().to_hyphenated().to_string();
        Shape {
            uuid:String::from(my_uuid),
            name:String::from(name),
            animations:Vec::new(),
            attributes:Attributes::new(),
        }
    }
    pub fn add_animation(&mut self,a:Animation){
        self.animations.push(a);
    }
    pub fn update(&mut self){
    println!("{}",self.attributes.get_bounding_rectangle_color());
    todo!("the shape ypdate fn");
    }
    
}//end of impl block