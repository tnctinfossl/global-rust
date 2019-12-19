pub use super::vec2rad::*;
use serde_derive::*;
use std::ops::Not;
pub static DIAMETOR_ROBOT: f32 = 0.10; //[m]

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq, Ord, Serialize, Deserialize)]
pub enum RobotID {
    Blue(u32),
    Yellow(u32),
}

impl Not for RobotID {
    type Output = RobotID;
    fn not(self) -> RobotID {
        use RobotID::*;
        match self {
            Blue(number) => Yellow(number),
            Yellow(number) => Blue(number),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Robot {
    pub position: Vec2Rad,
    pub diametor: f32,
}

impl Robot {
    #[allow(dead_code)]
    pub fn new(position: Vec2Rad, diametor: f32) -> Robot {
        Robot { position, diametor }
    }
}