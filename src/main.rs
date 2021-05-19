use bilzaa2d::State;

fn main(){
    let mut s = State::new();
    s.add_shape("nima");
    s.add_shape("shah");
    s.add_shape("oknice");
    s.add_shape("bell");
    s.add_shape("zin mate");
    s.add_animation(1,
        9,
        5,
        67,
        7,
        "counter",
        "height");
    s.draw();
    // let mut s:Shape = Shape::new(55, 63);
    // println!("{:?}",s);
}