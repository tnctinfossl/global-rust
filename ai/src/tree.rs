extern crate model;
use super::bitfield::BitField;
use super::evaluation::*;
use glm::*;
use model::*;
use std::iter::*;

/*pub fn tree(world:&World){

    //team交互に切り替え
    (..).map(tree_placement()).collect();
    tree_evalution(serch_renge());
    sort;

}*/

//fn tree_placement(team:&Team)->{}

fn tree_evalution(world:&World)->(f32,f32){

    
    space_domination(&world.blues,&world.yellows,&world.field)
}

/*fn serch_renge(prev_position:Vec2,current_position:Vec2)->{

}*/
