use bilzaa2d::Bilzaa2d;
use std::time::Duration;
use std::thread;
use bilzaa2dutil::{AttributesEnum};

fn main(){
    let mut b2d = Bilzaa2d::new();
    b2d.play_head.play();
    {
            let a = b2d.add_shape("nima");
            a.add_counter( 0,
                10,
                0,
                100,
                AttributesEnum::Width);
    }
    //=========================================
    // thread::sleep(Duration::from_millis(5000));
    //=========================================           
    b2d.update(5000);
    b2d.draw();
}