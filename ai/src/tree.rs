extern crate model;
use super::bitfield::BitField;
use glm::*;
use model::*;

pub fn decode(bitfield:&BitField,field:&Field,bit_coordinate_x:usize,bit_coordinate_y:usize)->Vec2{
    let x = bit_coordinate_x as f32 / bitfield.width() as f32;
    let y = bit_coordinate_y as f32 / bitfield.height() as f32;
    let rate = Vec2::new(x,y) ;
    let half = Vec2::new(0.5,0.5);
    
    field.outfield*(rate-half)
}