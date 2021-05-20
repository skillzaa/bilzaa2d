use bilzaa2d::Bilzaa2d;
fn main(){
    let mut b2d = Bilzaa2d::new();
    b2d.add_shape("nima");
    b2d.add_shape("shah");
    b2d.add_shape("oknice");
    b2d.add_shape("bell");
    b2d.add_shape("zin mate");
    b2d.add_animation(1,
        2,
        5,
        67,
        7,
        "counter",
        "height");
    b2d.draw();
    
}
