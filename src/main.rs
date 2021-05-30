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
       
    println!("Get Shape Test :: {:?}",b2d.get_shape("zeda"));    
    let jj = b2d.get_shape("zeda").unwrap();
    
    let c = jj.attributes.get_bounding_rectangle_color();
    println!("color :: {:?}",c);    
    
    /////////////////////////////
    let t  = b2d.get_shape("zeda")
    .unwrap().attributes.get_bounding_rectangle_color();
    println!("color :: {:?}",t);    
    // b2d.draw();
    // b2d.shapes
    
}
