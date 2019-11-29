extern crate model;
use super::bitfield::BitField;
use super::geometry::*;
use glm::*;
use model::*;
use std::iter::*;

fn encode(field: &Field, position: Vec2) -> (usize, usize) {
    let p = (field.outfield / 2.0 + position) / field.outfield;
    let x = min(p.x * 128.0, 127.0) as usize;
    let y = min(p.y * 100.0, 99.0) as usize;
    (x, y)
}
#[allow(dead_code)]
fn decode(
    bitfield: &BitField,
    field: &Field,
    bit_coordinate_x: usize,
    bit_coordinate_y: usize,
) -> Vec2 {
    let x = bit_coordinate_x as f32 / bitfield.width() as f32;
    let y = bit_coordinate_y as f32 / bitfield.height() as f32;
    let rate = Vec2::new(x, y);
    let half = Vec2::new(0.5, 0.5);

    field.outfield * (rate - half)
}

#[allow(dead_code)]
pub fn space_domination(my_team: &Team, enemy_team: &Team, field: &Field) -> (f32, f32) {
    let locate = |r: &Box<Robot>| encode(&field, r.position);
    let my_team_positions: Vec<_> = my_team.robots.iter().map(locate).collect();
    let enemy_team_positions: Vec<_> = enemy_team.robots.iter().map(locate).collect();

    let mut used_field = BitField::new();
    let mut my_team_field = BitField::new();
    let mut enemy_team_field = BitField::new();

    let merge = |ps: &Vec<(usize, usize)>, k: usize| -> BitField {
        ps.iter()
            .map(|p| BitField::new_rect(*p, k))
            .fold(BitField::new(), |x, y| x | y)
    };

    for i in 0..127 {
        let new_my_team = merge(&my_team_positions, i);
        let new_enemy_team = merge(&enemy_team_positions, i);

        let conflict = used_field.clone() | (new_my_team.clone() & new_enemy_team.clone());

        my_team_field |= new_my_team & !conflict.clone();
        enemy_team_field |= new_enemy_team & !conflict.clone();
        used_field |= my_team_field.clone() | enemy_team_field.clone() | conflict.clone();
    }
    let ret_b = my_team_field.count_one() as f32 / my_team_field.area() as f32;
    let ret_y = enemy_team_field.count_one() as f32 / enemy_team_field.area() as f32;
    (ret_b, ret_y)
}

//パスの可能性を評価する
#[allow(dead_code)]
pub fn passable<'a,I:Iterator<Item=&'a Vec2>>((begin,end):(Vec2,Vec2),objects:I)->f32{
    let nearest_distance=distance_path_nearest_points((begin,end), objects).unwrap_or(std::f32::MAX);
    let path_distance= distance(begin, end);
    println!("{},{}",nearest_distance,path_distance);
    -log(nearest_distance+1.0)-log(path_distance+1.0)
}

//シュートの可能性を評価する
#[allow(dead_code)]
pub fn shootable(field: &Field, mine: &Team, yours: &Team) -> f32 {
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
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_path_nearest_points() {
        let (begin, end) = (vec2(1.0, 1.0), vec2(3.0, 3.0));
        assert_eq!(distance_path_nearest_points((begin, end), [].iter()), None);
        assert_eq!(
            distance_path_nearest_points((begin, end), [vec2(2.0, 0.0)].iter()),
            Some(sqrt(2.0))
        );
        assert_eq!(
            distance_path_nearest_points((begin, end), [vec2(0.0, 2.0)].iter()),
            Some(sqrt(2.0))
        );
        assert_eq!(distance_path_nearest_points((begin, end), [vec2(0.0, 0.0)].iter()), None);
        assert_eq!(
            distance_path_nearest_points((end, begin), [vec2(2.0, 0.0)].iter()),
            Some(sqrt(2.0))
        );
        assert_eq!(
            distance_path_nearest_points((end, begin), [vec2(0.0, 2.0)].iter()),
            Some(sqrt(2.0))
        );
        assert_eq!(distance_path_nearest_points((end, begin), [vec2(0.0, 0.0)].iter()), None);
    }
}
