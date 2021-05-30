use bilzaa2d::Bilzaa2d;
fn main(){
    let mut b2d = Bilzaa2d::new();
    b2d.play_head.play();
    b2d.add_shape("nima");
    b2d.add_shape("jaman");
    b2d.add_shape("zeda");
    
    b2d.add_animation("zeda",
        2,
        5,
        67,
        7,
        "height",
        );
       
    println!("{:?}",b2d.play_head.time());    
    b2d.draw();
    // b2d.shapes
}
