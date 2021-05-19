// mod super::;
use super::Animation;

#[derive(Debug)]
pub struct Shape{
    pub name:String,
    pub no:u128,
    pub id:u128,
    pub animations:Vec<Animation>,
}