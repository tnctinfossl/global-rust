use super::bitfield::*;
use super::model::*;
use glm::*;


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
pub fn space_domination(mine: &[Vec2], yours: &[Vec2], field: &Field) -> (f32, f32) {
    let locate = |&p| encode(&field, p);
    let my_team_positions: Vec<_> = mine.iter().map(locate).collect();
    let enemy_team_positions: Vec<_> = yours.iter().map(locate).collect();

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

