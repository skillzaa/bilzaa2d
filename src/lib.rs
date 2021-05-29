mod shape;
// mod animation;
use std::collections::HashMap;
use bilzaa2dcounter::Animation;
use shape::Shape;
pub use playhead::PlayHead;
//===============================
#[derive(Debug)]
pub struct Bilzaa2d {

    pub    components:HashMap<String,Shape>, 
    pub    play_head:PlayHead,
    pub    shapes:Vec<Shape>,
}

impl Bilzaa2d {
    pub fn new()->Bilzaa2d{
        Bilzaa2d {
            components: HashMap::new(),
            play_head: PlayHead::new(100000, true),
            shapes: Vec::new(),
        }
    }
    pub fn add_shape(&mut self,name:&str){
        let s = shape::Shape::new(name);
        // println!("This is attributes.get_bounding_rectangle_color from lib {} ",s.attributes.get_bounding_rectangle_color());
        self.shapes.push(s);
    }
    pub fn add_comp(&mut self,name:&str){
        let v:Shape = shape::Shape::new(name);
        
        self.components.insert(String::from(name),v);
    }
    pub fn get_comp(){
        
    }
    pub fn draw(&self){
        println!("Here are the Shapes");
        for n in 0..101 {
            match self.shapes.get(n) {
            Some(x)=>println!("{}::{:?}",n, x),
            _=>(),
            }    
        }

        match self.components.get("comp01") {
            Some(x)=>println!("::{:?}",x),
            _=>(),
            }            
    }
    pub fn draw_comp(&self,name:&str){
        println!("Drawing component");
        match self.components.get(name) {
            Some(x)=>println!("::{:?}",x),
            _=>(),
            }            
    }
    
    pub fn add_animation(&mut self, idx:usize,from_second:u128,to_second:u128,from:u128,to:u128,attr_to_animate:&str){
       match self.shapes.get_mut(idx) {
            Some(s)=>{
                let a =  Animation::new(from_second,to_second,from,to,attr_to_animate);
                match  a {
                    Some(ani)=>s.add_animation(ani),
                    None=> panic!("!!!"),
                }
                
            },
            _=>(),
       }
    }
}//impl
//////////////////////////////////////////
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
