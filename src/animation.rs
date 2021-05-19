#[derive(Debug)]
pub struct Animation {
    pub from_second:i128,
    pub to_second:i128,
    pub from:i128,
    pub to:i128,
    pub generator:String,
    pub attr_to_animate:String,
}
//==========================================
impl Animation{
    pub fn new()->Animation{
        Animation {
            from_second:0,
            to_second:200,
            from:200,
            to:65000,
            generator:String::from("some gen"),
            attr_to_animate:String::from("width"),
            }
    }
}//end of impl block