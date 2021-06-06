#[derive(Debug)]
pub struct DataNode {
    c:char,
    f:u128,
    t:u128,
}

impl DataNode{
    
    pub fn new(c:char,f:u128,t:u128)->Option<DataNode>{
        Some(DataNode {
            c,
            f,
            t,
        })
    }



}