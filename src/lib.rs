#[derive(Debug)]
struct Shape{
    name:String,
    no:u128,
    id:u128,
}
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
            }
        )
    }
    pub fn draw(&self){
        println!("Here are the Shapes");
        for n in 0..101 {
            match self.shapes.get(n) {
            Some(x)=>println!("x {:?}", x),
            _=>(),
            }
            
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
