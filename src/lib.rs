
mod shape;
mod attributes;

mod animation;
use animation::Animation;
use shape::Shape;
//===============================
#[derive(Debug)]
pub struct Bilzaa2d {
    // shapes is a simple vec being managed by lib
pub    shapes:Vec<Shape>,
}

impl Bilzaa2d {
    pub fn new()->Bilzaa2d{
        Bilzaa2d {
            shapes: Vec::new(),
        }
    }
    pub fn add_shape(&mut self,name:&str){
        let mut s = shape::Shape::new(name);
        println!("This is attributes.get_bounding_rectangle_color from lib {} ",s.attributes.get_bounding_rectangle_color());
        self.shapes.push(s);
    }
    pub fn draw(&self){
        println!("Here are the Shapes");
        for n in 0..101 {
            match self.shapes.get(n) {
            Some(x)=>println!("{}::{:?}",n, x),
            _=>(),
            }
            
        }
    }
    pub fn add_animation(&mut self, idx:usize,from_second:u128,to_second:u128,from:u128,to:u128,generator:&str,attr_to_animate:&str){
       match self.shapes.get_mut(idx) {
            Some(s)=>{
                let a =  Animation::new(from_second,to_second,from,to,generator,attr_to_animate);
                match  a {
                    Ok(ani)=>s.add_animation(ani),
                    Err(e)=> panic!("{:?}",e),
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
