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

//パス通過性について超過する
//制約 objects にはbegin,endが含まれていないこと
#[allow(dead_code)]
pub fn pathable<I>((begin, end): (Vec2, Vec2), objects: I) -> f32
where
    I: Iterator<Item = Vec2>,
{
    /* 座標beginを通り、線分[begin,end]に垂直な直線fと
     ** 座標endを通り、線分[begin,end]に垂直な直線gに挟まれた領域に含まれる
     ** 座標郡objectsと線分[begin,end]の距離を求め、最短のものを返す
     */
    //[begin,end]の順番に制約がある場合が予想される。制約が発生していないか確認すること

    let d = normal(end - begin);
    let f = |p: Vec2| dot(p, d) - dot(d, begin);
    let g = |p: Vec2| dot(p, d) - dot(d, end);

    objects
        .filter(|p: &Vec2| {
            0.0 < f(*p) && g(*p) < 0.0 //あっているか要確認
        })
        .map(|p: Vec2| distance_line_point((begin, end), p))
        .fold(std::f32::MAX, |x: f32, y: f32| if x < y { x } else { y })
}

pub fn evaluate_shoot(field: &Field, mine: &Team, yours: &Team) -> f32 {
    //計算量O(n2)程度
    let goal = field.your_goal(mine);
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
