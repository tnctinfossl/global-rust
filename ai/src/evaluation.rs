extern crate model;
use model::*;
use rand;

let mut bit_width = Field.infield.x / 128.0;
let mut bit_hight = Field.infield.y / 100.0;

pub fn convert_bit(locate:Vec2,num:u32){
    let mut converted_x = (0..num).map(locate.x / bit_width).collect();
    let mut conberted_y = (0..num).map(locate.y / bit_hight).collect();
}

//pub fn airspace_possession()