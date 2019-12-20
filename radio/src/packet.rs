use glm::*;
use serde_derive::*;
use std::collections::HashMap;
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Command {
    //座標
    pub position: Vec2,
    pub angle: f32,
    //キック動作
    pub kick: Option<f32>,
    pub chip: Option<f32>,
    pub dribbble: f32,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum RobotID {
    Blue(u32),
    Yellow(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet {
    //状況
    pub robots: Vec<(Vec2, f32)>,
    pub ball: Option<Vec2>,
    //命令
    pub commands: HashMap<RobotID, Command>,
}
