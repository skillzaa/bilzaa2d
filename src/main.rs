use bilzaa2d::{PlayHead,Bilzaa2d};
fn main(){
    let mut ph = PlayHead::new(100000, true);
    ph.play();
    let mut b2d = Bilzaa2d::new();
    b2d.add_shape("nima");
    b2d.add_comp("comp01");
    
    b2d.add_animation(0,
        2,
        5,
        67,
        7,
        "height",
        );
       
        
    println!("{:?}",ph.time());    
    b2d.draw();
    // b2d.shapes
}
