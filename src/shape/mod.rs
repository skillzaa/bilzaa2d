/**Making Animation is not its job that is done out side by lib and animation mod. shape just has to add it */
use uuid::Uuid;
use bilzaa2dutil::{Animatable, AnimateResponses, Attributes, AttributesEnum, BaseCounter};
mod datanode;
use datanode::DataNode;

#[derive(Debug)]
// #[derive(PartialEq)]
pub struct Shape{
           uuid:String,
           name:String,
           animations : Vec<Box<dyn Animatable>>,
           data : Vec<DataNode>,
    pub    attr:Attributes,
}
//==========================================
impl Shape{
    pub fn new(name:&str)->Shape{
        let my_uuid = Uuid::new_v4().to_hyphenated().to_string();
        Shape {
            uuid:String::from(my_uuid),
            name: name.to_string(),
            animations:Vec::new(),
            data:Vec::new(),
            attr:Attributes::new(),
        }
    }
    pub fn add_animation(&mut self,ani: impl Animatable + 'static)->Option<bool>{
       self.animations.push(Box::new(ani));
       Some(true)       
    }
    pub fn add_fake_counter(&mut self)->Option<bool>{
        let ani = BaseCounter::new(0.0,
        5.0,1,100,AttributesEnum::Height)?;
        self.animations.push(Box::new(ani));
        Some(true)
    }
    
    pub fn update(&mut self,time:u128)->Option<u128>{    
        
        for ani in self.animations.iter() {
            // let valid = ani.is_time_valid(time)
                let new_value = ani.animate(time)?;
                let attr_to_animate = ani.get_attr_to_animate();
                    match attr_to_animate {
                        AttributesEnum::Opacity=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attr.set_opacity(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::X=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attr.set_x(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::Y=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attr.set_y(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::Width=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attr.set_width(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::Height=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attr.set_height(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::StartAngle=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attr.set_start_angle(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::LineWidth=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attr.set_line_width(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::ShadowBlur=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attr.set_shadow_blur(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::ShadowOffsetX=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attr.set_shadow_offset_x(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::ShadowOffsetY=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attr.set_shadow_offset_y(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::LineDashGap=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attr.set_line_dash_gap(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::LineDashSize=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attr.set_line_dash_size(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::BoundingRectanglePadding=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attr.set_bounding_rectangle_padding(u);
                                },
                                _=>(),
                            }
                        },
                        _=>return None,
                    }                        
        }  
    return None;      
    }
    
    
}//end of impl block

fn get_draw_data()->DataNode{
todo!();
}