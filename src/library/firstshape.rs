use bilzaa2d::Shape;
// use bilzaa2d:
// use super
pub fn first_shape(shape_name:&str)->Shape{
    let mut shape = Shape::new(shape_name);
   
    shape.add_fake_counter();
    shape.attr.get_bounding_rectangle_padding();
    shape
}
