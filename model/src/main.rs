extern crate glm;
mod bitfield;
mod model;
use bitfield::BitField;
fn main() {

    let mut a =BitField::new();
    a.rect((2,2),1);
    let mut b =BitField::new();
    b.write((0,0),true).write((0,1),true);
    println!("{}",a.dump());

}
