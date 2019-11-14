extern crate model;
use model::*;
use rand;
use super::bitfield::BitField;
use glm::*;
#[feature(test)]


pub fn encode(field: &Field,position: Vec2)->(usize,usize){
    let p = (field.outfield / 2.0 + position) / field.outfield;
    let x = min(p.x * 128.0,127.0) as usize;
    let y = min(p.y * 100.0,99.0) as usize;
    (x,y)
}

pub fn space_domination (world:&World) -> (f32,f32){
    let locate = |r:&Box<Robot>|{
        encode(&world.field, r.position)
    }    ;
    let blue_positions:Vec<_> = world.blues.robots.iter().map(locate).collect();
    let yellow_positions:Vec<_> = world.yellows.robots.iter().map(locate).collect();

    let mut used_field = BitField::new();
    let mut blue_field = BitField::new();
    let mut yellow_field = BitField::new();

    let merge = |ps:&Vec<(usize,usize)>,k:usize|->BitField{
        ps.iter().map(|p| BitField::new_rect(*p, k)).fold(BitField::new(),|x,y| x|y)
    };

    for i in 0..127{
        let new_blue = merge(&blue_positions,i);
        let new_yellow = merge(&yellow_positions,i);

        let conflict = used_field.clone() | (new_blue.clone() & new_yellow.clone()) ;

        blue_field |= new_blue & !conflict.clone();
        yellow_field |= new_yellow & !conflict.clone();
        used_field |= blue_field.clone() | yellow_field.clone() | conflict.clone();

    }
    let ret_b = blue_field.count_one() as f32 / blue_field.area() as f32;
    let ret_y = yellow_field.count_one() as f32 /  yellow_field.area() as f32;
    (ret_b,ret_y)
}

fn max(a:f32,b:f32)->f32{
    if a<b {
        b
    }
    else{
        a
    }
}

fn min(a:f32,b:f32)->f32{
    if a>b {
        b
    }
    else{
        a
    }
}
