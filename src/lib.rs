mod shape;
// mod animation;
use std::collections::HashMap;
use bilzaa2dcounter::Animation;
use shape::Shape; //do not make this public
use playhead::PlayHead;
//===============================
#[derive(Debug)]
pub struct Bilzaa2d {

    pub    shapes:HashMap<String,Shape>, 
    pub    play_head:PlayHead,
}

impl Bilzaa2d {
    pub fn new()->Bilzaa2d{
        Bilzaa2d {
            shapes: HashMap::new(),
            play_head: PlayHead::new(100000, true),
        }
    }
    pub fn add_shape(&mut self,name:&str){
        let shp:Shape = shape::Shape::new(name);
        self.shapes.insert(String::from(name),shp);

        // println!("This is attributes.get_bounding_rectangle_color from lib {} ",shp.attributes.get_bounding_rectangle_color());
    }
    
    pub fn update(){
        todo!();
    }
    pub fn draw(&self){
        for (name, shape) in &self.shapes {
            println!("Name:: {:?} Shape:: {:?}", name, shape);
        }
    }
    
    pub fn add_animation(&mut self, name:&str,from_second:u128,to_second:u128,from:u128,to:u128,attr_to_animate:&str){
       match self.shapes.get_mut(name) {
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
