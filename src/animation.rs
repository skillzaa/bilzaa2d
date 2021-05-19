#[derive(Debug)]
pub struct Animation {
    pub from_second:u128,
    pub to_second:u128,
    pub from:u128,
    pub to:u128,
    pub generator:String,
    pub attr_to_animate:String,
}
//==========================================
impl Animation{
    pub fn new(from_second:u128,to_second:u128,from:u128,to:u128,generator:&str,attr_to_animate:&str)->Animation{
        Animation {
            from_second,
            to_second,
            from,
            to,
            generator:String::from(generator),
            attr_to_animate:String::from(attr_to_animate),
            }
    }
}//end of impl block