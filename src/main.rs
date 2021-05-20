use bilzaa2d::Bilzaa2d;
fn main(){
    let mut b2d = Bilzaa2d::new();
    b2d.add_shape("nima");
    b2d.add_comp("comp01");
    b2d.add_comp("comp02");
    
    b2d.add_animation(0,
        2,
        5,
        67,
        7,
        "counter",
        "height");
    b2d.draw();
    b2d.draw_comp("comp01");
    b2d.draw_comp("comp02");
    
}
