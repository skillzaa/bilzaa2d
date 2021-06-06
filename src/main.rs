use bilzaa2d::BaseCounter;
use bilzaa2d::{Bilzaa2d,AttributesEnum,};
// use bilzaa2dutil::{AttributesEnum};
use std::thread;
use std::collections::HashMap;
use std::time::Duration;
use bilzaa2d::Shape;
fn main(){
    let mut b2d = Bilzaa2d::new();
    b2d.play_head.play();
    let mut shapes = HashMap::new();
    /* ok some thing better */
    let mut shape = Shape::new("aaa");
    shape.add_fake_counter();
    shape.attr.get_bounding_rectangle_padding();
    ////////////////////////////////////
    let mut shapeb = Shape::new("bbb");
    shapeb.add_fake_counter();
    shapeb.add_fake_counter();
    shapeb.add_fake_counter();

    shapeb.attr.set_height(55);
    let a  = shapeb.attr.get_height();
    assert_eq!(a,55);    
    let mut shapec = Shape::new("ccc");
    shapec.add_fake_counter();

    shapes.insert(String::from("aaa"), shape);
    shapes.insert(String::from("bbb"), shapeb);
    shapes.insert(String::from("ccc"), shapec);
    
    //--finally
    b2d.shapes = shapes;
    // let a = b2d.add_shape("one");
    // let b = b2d.add_shape("two");
    // let c = b2d.add_shape("three");
    // let d = b2d.add_shape("four");

    // let c = b2d.get_counter(0.0, 10.0, 0, 100,AttributesEnum::Height);
//    b2d.add_shape(a);
    b2d.draw();
    todo!();
}
fn add_shape(b2d:&mut Bilzaa2d,shape_name:&str)->String{
    b2d.add_shape(shape_name);
    shape_name.to_string()
}
//fn build_app(b2d:&mut Bilzaa2d){

    
    
//     {
//         let counter_option = b2d.get_counter( 0.0,
//         10.0,
//         0,
//         100,
//         AttributesEnum::Height);
    
//         let counter = match counter_option {
//         Some(aaa)=> aaa,
//         None=>panic!("Failed to create Animation objected"),
//     };

// }
      
//     //=========================================
//      thread::sleep(Duration::from_millis(5000));
//     //=========================================           
//     b2d.update(5000);
//     b2d.draw();

//}
