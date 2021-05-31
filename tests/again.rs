use bilzaa2d::Bilzaa2d;
use std::time::Duration;
use std::thread;
use bilzaa2dattributes::AttributesEnum;

#[cfg(test)]
#[test]
fn full_test(){
//This test can fail since time in calc may change    
    let mut b2d = Bilzaa2d::new();
    b2d.play_head.play();
    {
            let a = b2d.add_shape("nima");
            a.add_animation( 0,
                10,
                0,
                500,
                AttributesEnum::Width);
    }
    //=========================================
    thread::sleep(Duration::from_millis(5000));
    //=========================================           
    b2d.update(b2d.play_head.time());

    match b2d.get_shape("nima"){
        Some(s)=>{
            let w = s.attributes.get_width();
            assert_eq!(w,250);

        },
        None=>panic!("failed"),
    }   
   
}

#[cfg(test)]
#[test]
fn full_second(){
//This test can fail since time in calc may change    
    let mut b2d = Bilzaa2d::new();
    b2d.play_head.play();
    {
            let a = b2d.add_shape("nima");
            a.add_animation( 0,
                20,
                0,
                1000,
                AttributesEnum::Width);
    }
    //=========================================
    thread::sleep(Duration::from_millis(5000));
    //=========================================           
    b2d.update(b2d.play_head.time());

    match b2d.get_shape("nima"){
        Some(s)=>{
            let w = s.attributes.get_width();
            assert_eq!(w,250);
        },
        None=>panic!("failed"),
    }   
   
}
#[cfg(test)]
#[test]
fn full_reverse(){
//This test can fail since time in calc may change    
    let mut b2d = Bilzaa2d::new();
    b2d.play_head.play();
    {
            let a = b2d.add_shape("nima");
            a.add_animation( 0,
                20,
                1000,
                0,
                AttributesEnum::Width);
    }
    //=========================================
    thread::sleep(Duration::from_millis(5000));
    //=========================================           
    b2d.update(b2d.play_head.time());

    match b2d.get_shape("nima"){
        Some(s)=>{
            let w = s.attributes.get_width();
            assert_eq!(w,750);
        },
        None=>panic!("failed"),
    }   
   
}