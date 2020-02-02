extern crate model;
use model::*;
pub mod field;
pub mod geometry;
pub mod pass;

pub mod bitfield;
pub mod mattrix;
pub use super::game::robot::*;
pub use super::game::scene::*;
pub use field::space_domination;
pub use field::*;
pub use geometry::*;
use glm::*;
//パスの可能性を評価する
#[allow(dead_code)]
pub fn passable<'a, I: Iterator<Item = &'a Vec2>>((begin, end): (Vec2, Vec2), objects: I) -> f32 {
    let nearest_distance =
        distance_path_nearest_points((begin, end), objects).unwrap_or(std::f32::MAX);
    let path_distance = distance(begin, end);
   // println!("{},{}", nearest_distance, path_distance);
    -log(nearest_distance + 1.0) - log(path_distance + 1.0)
}



//シュートの可能性を評価する
/*#[allow(dead_code)]
pub fn shootable(_field: &Field, _mine: &Team, _yours: &Team) -> f32 {
    //計算量O(n2)程度
    //let goal = field.your_goal(mine);
    /*
      mine.robots
          .iter()
          .map(|target: &Box<Robot>| {
              let others = mine
                  .robots
                  .iter()
                  .filter(|cmp: &&Box<Robot>| cmp.id != target.id)
                  .chain(yours.robots.iter());
              let distance = others
                  .map(|other: &Box<Robot>| {
                      distance_segment_point((target.position, goal), other.position)
                  })
                  .reduce(util::min)
          })
    */
    0.0 //準備中
}*/
