#[derive(Debug)]
pub struct Animation {
    pub from_second:i128,
    pub to_second:i128,
    pub from:i128,
    pub to:i128,
    pub generator:String,
    pub attr_to_animate:String,
}
