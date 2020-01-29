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
    println!("{},{}", nearest_distance, path_distance);
    -log(nearest_distance + 1.0) - log(path_distance + 1.0)
}

//Sceneをもらってパスを評価
/*#[allow(dead_code)]
pub fn scene_passable(scene: &Scene) -> f32 {
    let mut blue_points: Vec<_> = Vec::new();
    let mut yellow_points: Vec<_> = Vec::new();
    for (id, robot) in scene.robots {
         match id {
            RobotID::Blue(_) => blue_points.push(robot.position),
            RobotID::Yellow(_) => yellow_points.push(robot.position),
            }
    }
    let blue_vec2 = blue_points.iter().map(|bp|bp.to_vec2());
    let ave = Vec::new();
    for positions in blue_vec2{
        ave.push(passable((positions[0],positions[1]),positions ));
    }*/

/*for i in 0..4 {
        scores.push(passable(
            (
                scene
                    .robots
                    .get(&RobotID::Blue(i))
                    .unwrap()
                    .position
                    .to_vec2(),
                scene
                    .robots
                    .get(&RobotID::Blue(i + 1))
                    .unwrap()
                    .position
                    .to_vec2(),
            ),
            [positions.pop().unwrap(),positions.pop().unwrap(),positions.pop().unwrap(),positions.pop().unwrap(),positions.pop().unwrap()].iter()
        ));
    ;
    ave
}*/

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
