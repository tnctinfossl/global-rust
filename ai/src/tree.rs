extern crate model;
use super::bitfield::BitField;
use super::evaluation::*;
use glm::*;
use model::*;
use std::iter::*;
use rand::Rng;
use rand_distr::{Normal,Distribution};

/*pub fn tree(world:&World){
    world.
    //team交互に切り替え

}

fn tree_placement<R:Rng>(rng: &mut R,team:&Team)->Vec2{
    let mut count = 0.0;
    let mut time = count * 0.001;//1ms
    let fastest_v = 10.0;
    let mut prev_position = Vec2::new(0.0,0.0);
    let current_position =  &team.robots.;
    let mut v = current_position - prev_position; 
    let q = Normal::new(current_position - v*time,fastest_v / time);
    count += 1.0;
}

fn tree_evalution(world:&World)->(f32,f32){

    
    space_domination(&world.blues,&world.yellows,&world.field)
}*/
