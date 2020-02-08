use super::*;
use glm::*;
use lazy_static::*;
lazy_static! {
    pub static ref FIELD_A: Rectangle =
        rectangle(vec2(-13.400 / 2.0, 10.400 / 2.0), vec2(13.400, -10.400));
    pub static ref FIELD_B: Rectangle =
        rectangle(vec2(-10.400 / 2.0, 7.400 / 2.0), vec2(10.400, -7.400));
    pub static ref GOALS_A: (Vec2, Vec2) = (vec2(-13.400 / 2.0, 0.0), vec2(13.400 / 2.0, 0.0));
    pub static ref GOALS_B: (Vec2, Vec2) = (vec2(-10.400 / 2.0, 0.0), vec2(10.400 / 2.0, 0.0));
}
