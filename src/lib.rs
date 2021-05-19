
mod shape;
//--if the fol 2 lines r removed the animation in shape mod disappear
mod animation;
use animation::Animation;
use shape::Shape;
//===============================
#[derive(Debug)]
pub struct State {
    //shapes is a simple vec being managed by lib
    shapes:Vec<Shape>,
    flag:bool,    
}

impl State {
    pub fn new()->State{
        State {
            shapes: Vec::new(),
            flag:true,
        }
    }
    pub fn add_shape(&mut self,name:&str){
        let s = shape::Shape::new(name);
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
                s.add_animation(a);
            },
            _=>(),
       }
    }
}//impl
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
