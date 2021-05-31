use bilzaa2d::Bilzaa2d;
fn main(){
    let mut b2d = Bilzaa2d::new();
    
    b2d.play_head.play();
    let a = b2d.add_shape("nima");
    // println!("Shape ======>>>>>>>{:?}",a);    

    a.add_animation( 0,
        10,
        100,
        0,
        "width");

        for i in 0..10000000 {
            let n = i+ 543;
            let s = String::from("waste some time");
            
        }
    println!("Time ======>>>>>>>{:?}",b2d.play_head.time());    
    b2d.update(b2d.play_head.time());
    b2d.draw();    
        
}

