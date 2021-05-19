
mod shape;
mod animation;
use animation::Animation;
use shape::Shape;
//===============================
#[derive(Debug)]
pub struct State {
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
        self.shapes.push(
            Shape {
                name: String::from(name),
                no:444,
                id:925,
                animations:Vec::new(),
            }
        )
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
    pub fn add_animation(&mut self){
       match self.shapes.get_mut(0) {
            Some(x)=>{
                let a = Animation  {
                    from_second:0,
                    to_second:100,
                    from:200,
                    to:660000,
                    generator:String::from("generator"),
                    attr_to_animate:String::from("width"),
                };
                x.animations.push(a);
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
