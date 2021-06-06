
// mod app;
// pub use app::Bilzaa2d;
// mod shape;
// pub use shape::Shape;
// pub use bilzaa2dutil::AttributesEnum;//pub


pub use bilzaa2dutil::{Animatable,AttributesEnum};
mod shape;
pub use bilzaa2dutil::BaseCounter;
use std::collections::HashMap;
pub use shape::Shape; //do not make this public
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
    pub fn add_shape(&mut self,shape_name:&str)->String{
        let shp:Shape = Shape::new(shape_name);
        self.shapes.insert(String::from(shape_name),shp);
        String::from(shape_name)   
    }
    pub fn update(&mut self, time:u128){
        
        for (_, shp) in self.shapes.iter_mut() {
            shp.update(time);
        }
          
    }
    pub fn draw(&self){
        for (name, shape) in &self.shapes {
            println!("=========================={:?}===========",&name);
            println!("Name:: {:?} Shape:: {:?}", name, shape);
        }
    }
    pub fn get_shape(&mut self,shape_name:String)->Option<& mut Shape>{
        let s_slice: &str = &shape_name[..];
        let s = self.shapes.get_mut(s_slice);
        match s {
            Some(ss)=> return Some(ss),
            None=> return None,
        }
    }
    pub fn add_animation(&mut self,shape_name:&str,animation: impl Animatable + 'static)->Option<bool>{
        let s = self.get_shape(shape_name.to_owned())?;
        s.add_animation(animation);
        Some(true)
    }
    pub fn get_counter(self,from_second:f64,to_second:f64,from:u128,to:u128,attr_to_animate:AttributesEnum)->Option<BaseCounter>{
        let c = BaseCounter::new(from_second,
            to_second,from,to,attr_to_animate);
        match c {
            Some(ani)=> Some(ani),
            None=>None,
        }
    }
}//impl
//////////////////////////////////////////
#[cfg(test)]
#[test]
fn bilzaa2d_new() {
    let mut b2d = Bilzaa2d::new();
    let a = b2d.add_shape("nima");
    let w = a.attributes.get_width();
    assert!(w > 0);
}

//from_second:u128,to_second:u128,from:u128,to:u128,attr_to_animate:AttributesEnum