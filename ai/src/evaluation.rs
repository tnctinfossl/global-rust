extern crate model;
use model::*;
use rand;

let mut bit_width = Field.infield.x / 128.0;
let mut bit_hight = Field.infield.y / 100.0;

pub fn convert_bit(x,y,count){
    let mut conberted_x = World.blues.robots.position.x / bit_width;
    let mut conberted_y = World.blues.robots.position.y / bit_hight;

}
