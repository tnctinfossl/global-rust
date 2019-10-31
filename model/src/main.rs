extern crate glm;
mod bitfield;
mod model;
use bitfield::BitField;
fn main() {

    let mut a =BitField::new_rect((3,3),2);
    let mut b =BitField::new();
    b.write((0,0),true).write((4,2),true);
    println!("{}",(a&b).dump());

}
