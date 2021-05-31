use bilzaa2d::Bilzaa2d;
use std::time::Duration;
use std::thread;
use bilzaa2dattributes::AttributesEnum;

fn main(){
    let mut b2d = Bilzaa2d::new();
    b2d.play_head.play();
    {
            let a = b2d.add_shape("nima");
            a.add_animation( 0,
                20,
                0,
                1000,
                AttributesEnum::LineDashSize);
    }
    //=========================================
    thread::sleep(Duration::from_millis(5000));
    //=========================================           
    b2d.update(b2d.play_head.time());
    b2d.draw();
}