use bilzaa2d::Bilzaa2d;
fn main(){
    let mut b2d = Bilzaa2d::new();
    
    //b2d.play_head.play();
    let a = b2d.add_shape("nima");
    a.add_animation( 0,
        3,
        0,
        70,
        "width");

    b2d.update(1500);
    b2d.draw();    
        
}

