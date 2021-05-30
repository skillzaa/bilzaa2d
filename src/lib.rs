mod shape;
// mod animation;
use std::collections::HashMap;
use bilzaa2dcounter::Animation;
use shape::Shape; //do not make this public
use playhead::PlayHead;
//===============================
#[derive(Debug)]
pub struct Bilzaa2d {

           shapes:HashMap<String,Shape>, 
    pub    play_head:PlayHead,
}

impl Bilzaa2d {
    pub fn new()->Bilzaa2d{
        Bilzaa2d {
            shapes: HashMap::new(),
            play_head: PlayHead::new(100000, true),
        }
    }
    pub fn add_shape(&mut self,shape_name:&str)->&mut Shape{
        let shp:Shape = shape::Shape::new(shape_name);
        self.shapes.insert(String::from(shape_name),shp);
        //-------------
        match self.get_shape(shape_name) {
            Some(a)=> return a,
            None=>panic!("Could not create or retrieve shape"),
        }
   
    }
    pub fn update(){
        todo!();
    }
    pub fn draw(&self){
        for (name, shape) in &self.shapes {
            println!("=========================={:?}===========",name);
            println!("Name:: {:?} Shape:: {:?}", name, shape);
        }
    }
    pub fn get_shape(&mut self,shape_name:&str)->Option<& mut Shape>{
        let s = self.shapes.get_mut(shape_name);
        match s {
            Some(ss)=> return Some(ss),
            None=> return None,
        }
    }
    //--idea dropped since it may not be correct idea
    // pub fn set_attr(shape_name:&str,attr_name:&str,value:&str){
    //     todo!();
    // }
    
    // pub fn get_attr(){
    //     todo!();
    // }
}//impl
//////////////////////////////////////////
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
