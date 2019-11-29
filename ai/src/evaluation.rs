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
pub fn passable((begin, end): (Vec2, Vec2), objects:&[Vec2]) -> f32
{
    /* 座標beginを通り、線分[begin,end]に垂直な直線fと
     ** 座標endを通り、線分[begin,end]に垂直な直線gに挟まれた領域に含まれる
     ** 座標郡objectsと線分[begin,end]の距離を求め、最短のものを返す
     */
    //[begin,end]の順番に制約がある場合が予想される。制約が発生していないか確認すること

    //[begin,end]を通る直線は ax+by+c=0を満たす
    let d = end-begin;
    let (a,b)=(d.x,-d.y);
    let c = -(a*begin.x+b*begin.y);
    //println!("{},{},{}",a,b,c);
    
    //[begin,end]の法線かつbeginを通る直線fを求める
    let fa = b;
    let fb = -a;
    let fc = -(fa*begin.x+fb*begin.x);
    let f = |p: Vec2| fa*p.x+fb*p.y+fc;
    println!("{},{},{}",fa,fb,fc);

    //[begin,end]の法線かつendを通る直線gを求める
    let ga =b;
    let gb =-a;
    let gc = -(ga*end.x+gb*end.x);
    let g = |p: Vec2| ga*p.x+gb*p.y+gc;
    println!("{},{},{}",ga,gb,gc);

    objects.iter()
        .filter(|p: &&Vec2| {
            println!("({},{})",g(**p),f(**p));
            0.0 < g(**p) && f(**p) < 0.0 //あっているか要確認
        })
        .map(|p: &Vec2| distance_line_point((begin, end), *p))
        .fold(std::f32::MAX, |x: f32, y: f32| if x < y { x } else { y })
}

#[test]
fn test_passable(){
    let objects=vec![vec2(0.5,1.0),vec2(0.5,-1.0)];
    let responce = passable((vec2(0.0,0.0),vec2(1.0,0.0)),&objects);
    assert_eq!(responce,1.0);
}



pub fn evaluate_shoot(field: &Field, mine: &Team, yours: &Team) -> f32 {
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
