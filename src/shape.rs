/**Making Animation is not its job that is done out side by lib and animation mod. shape just has to add it */
use uuid::Uuid;
use bilzaa2dutil::{BaseCounter,Animatable,AnimateResponses,AttributesEnum};

use bilzaa2dattributes::{Attributes};


#[derive(Debug)]
pub struct Shape{
           uuid:String,
           name:String,
        // animations:Vec<BaseCounter>,
           animations : Vec<Box<dyn Animatable>>,
    pub    attributes:Attributes,
}
//==========================================
impl Shape{
    pub fn new(name:&str)->Shape{
        let my_uuid = Uuid::new_v4().to_hyphenated().to_string();
        Shape {
            uuid:String::from(my_uuid),
            name:String::from(name),
            animations:Vec::new(),
            attributes:Attributes::new(),
        }
    }
    pub fn add_counter(&mut self,from_second:u128,to_second:u128,from:u128,to:u128,attr_to_animate:AttributesEnum){
        let a =  BaseCounter::new(from_second,to_second,
            from,to,attr_to_animate);
        match a {
            Some(aa)=>self.animations.push(Box::new(aa)),
            None=>panic!("Failed to create an animation"),
        }
        
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
                                    self.attributes.set_opacity(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::X=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attributes.set_x(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::Y=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attributes.set_y(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::Width=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attributes.set_width(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::Height=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attributes.set_height(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::StartAngle=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attributes.set_start_angle(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::LineWidth=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attributes.set_line_width(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::ShadowBlur=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attributes.set_shadow_blur(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::ShadowOffsetX=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attributes.set_shadow_offset_x(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::ShadowOffsetY=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attributes.set_shadow_offset_y(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::LineDashGap=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attributes.set_line_dash_gap(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::LineDashSize=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attributes.set_line_dash_size(u);
                                },
                                _=>(),
                            }
                        },
                        AttributesEnum::BoundingRectanglePadding=>{
                            match new_value {
                                AnimateResponses::U128(u)=>{
                                    self.attributes.set_bounding_rectangle_padding(u);
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