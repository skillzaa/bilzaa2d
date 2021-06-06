use bilzaa2d::Bilzaa2d;
use bilzaa2d::first_shape;
fn main(){
    let mut b2d = Bilzaa2d::new();
    b2d.insert_shape("aaa", first_shape("aaa"));
    b2d.insert_shape("bbb", first_shape("aaa"));
    
    b2d.draw();
}
