pub use bilzaa2dutil::{Animatable,AttributesEnum};
mod shape;
mod library;
pub use bilzaa2dutil::BaseCounter;
use std::collections::HashMap;
pub use shape::Shape; //do not make this public
use playhead::PlayHead;
pub use library::first_shape;
//===============================

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
    pub fn insert_shape(&mut self,shape_name:&str,shape:Shape)->Option<bool>{
        self.shapes.insert(String::from(shape_name), shape)?;
        Some(true)           
    }
    pub fn add_shape(&mut self,shape_name:&str)->String{
        let shp:Shape = Shape::new(shape_name);
        self.shapes.insert(String::from(shape_name),shp);
        String::from(shape_name)   
    }
    pub fn get_shape_ptr(&mut self,shape_name:&str)->Option<& mut Shape>{
        let s_slice: &str = &shape_name[..];
        let s = self.shapes.get_mut(s_slice);
        match s {
            Some(ss)=> return Some(ss),
            None=> return None,
        }
    }
    //----
    pub fn insert_animation(&mut self,shape_name:&str,animation: impl Animatable + 'static)->Option<bool>{
        let s = self.get_shape_ptr(shape_name.clone())?;
        s.add_animation(animation);
        Some(true)
    }
    pub fn add_counter(&mut self,shape_name:&str,from_second:f64, to_second:f64, from:u128, to:u128, attr_to_animate:AttributesEnum)->Option<bool>{
        let s = self.get_shape_ptr(shape_name.clone())?;
        let c = get_counter(from_second, to_second, from, to, attr_to_animate)?;
        s.add_animation(c);
        Some(true)
    }
    //----
    pub fn draw(&self){
        for (name, shape) in &self.shapes {
            println!("=========================={:?}===========",&name);
            println!("Name:: {:?} Shape:: {:?}", name, shape);
        }
    }
    pub fn update(&mut self, time:u128){
        
        for (_, shp) in self.shapes.iter_mut() {
            shp.update(time);
        }
          
    }
}//impl

pub fn get_counter(from_second:f64,to_second:f64,from:u128,to:u128,attr_to_animate:AttributesEnum)->Option<BaseCounter>{
    let c = BaseCounter::new(from_second,
        to_second,from,to,attr_to_animate);
    match c {
        Some(ani)=> Some(ani),
        None=>None,
    }
}

