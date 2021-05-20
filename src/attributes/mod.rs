
mod attribvalues;
use attribvalues::AttribValues;
#[derive(Debug)]
pub enum BnsEnum {B,N,S,}
#[derive(Debug)]
pub struct Bns {
    b:bool,
    n:u128,
    s:String,
    t:BnsEnum,
}
impl Bns {
    pub fn new(b:bool,n:u128,s:String,t:BnsEnum)->Bns{
        Bns {
        b,
        n,
        s,
        t,
        }
    }
}
#[derive(Debug)]

pub struct Attributes{
    //--strings
    bounding_rectangle_color:String,
    shadow_color:String,
    name:String,
    color:String,
    //Flags
    clockwise:bool,
    opacity:u128,
    filled:bool,
    //Numbers
    x:u128,
    y:u128,
    width:u128,
    height:u128,
    start_angle:u128,
    line_width:u128,
    shadow_blur:u128,
    shadow_offset_x:u128,
    shadow_offset_y:u128,
    line_dash_size:u128,
    line_dash_gap:u128,
    draw_bounding_rectangle:u128,
    bounding_rectangle_padding:u128,
}
impl Attributes {
    pub fn new()->Self{
        Attributes {
    bounding_rectangle_color: String::from("red"),
    shadow_color:String::from("red"),
    name:String::from("red"),
    color:String::from("red"),
    clockwise:true,
    filled:true,
    opacity:3200,
    x:3200,
    y:3200,
    width:3200,
    height:3200,
    start_angle:3200,
    line_width:3200,
    shadow_blur:3200,
    shadow_offset_x:3200,
    shadow_offset_y:3200,
    line_dash_size:3200,
    line_dash_gap:3200,
    draw_bounding_rectangle:3200,
    bounding_rectangle_padding:3200,
        }
    }
    pub fn set_width(&mut self,v:u128)->bool{
        self.width = v;
        true
    }

   
}//impl Attributes

