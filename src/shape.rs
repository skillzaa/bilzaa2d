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
    pub fn add_animation(&mut self,from_second:u128,to_second:u128,from:u128,to:u128,attr_to_animate:&str){
        let a =  Animation::new(from_second,to_second,from,to,attr_to_animate);
        match a {
            Some(aa)=>self.animations.push(aa),
            None=>panic!("Failed to create an animation"),
        }
        
    }
    pub fn update(&mut self,time:u128)->Option<u128>{
    // println!("{}",self.attributes.get_bounding_rectangle_color());
    
        for ani in self.animations.iter() {
            // let ata = ani.get_attr_to_animate();
            match ani.animate(time) {
                Some(new_value)=>{
                    println!("new attrib vlalue : {:?}",new_value);
                    self.attributes.set_width(new_value);
                    ()
                    
                },
                None=> return None,
            }
        }
    return None    
    }
    
}//end of impl block