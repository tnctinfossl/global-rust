use glm::*;
use serde_derive::*;
use std::collections::*;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum TeamID {
    Blue,
    Yellow,
}

impl Default for TeamID {
    fn default() -> TeamID {
        TeamID::Blue
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Goal {
    Right,
    Left,
}

impl Default for Goal {
    fn default() -> Goal {
        Goal::Right
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum RobotID {
    Blue(u32),
    Yellow(u32),
}

impl Default for RobotID {
    fn default() -> RobotID {
        RobotID::Blue(0)
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Robot {
    pub position: Vec2,
    pub angle: f32,
}

impl Default for Robot {
    fn default() -> Robot {
        Robot {
            position: vec2(0.0, 0.0),
            angle: 0.0,
        }
    }
}

impl Robot {
    #[allow(dead_code)]
    pub fn new(position: Vec2, angle: f32) -> Robot {
        Robot {
            position: position,
            angle: angle,
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Command {
    MoveAt(Vec2),
    MoveTo(RobotID),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet {
    //現在の状況
    pub robots: HashMap<RobotID, Robot>,
    pub ball: Option<Vec2>,
    pub team: TeamID,
    pub goal: Goal,
    //命令
    pub command_lists: HashMap<RobotID, Vec<Command>>,
}

impl Default for Packet {
    fn default() -> Packet {
        Packet {
            robots: HashMap::new(),
            ball: None,
            team: TeamID::default(),
            goal: Goal::default(),
            command_lists: HashMap::new(),
        }
    }
}
