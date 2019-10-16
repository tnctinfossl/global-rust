use glm::Vec2;
use std::collections::HashMap;
use std::time;
#[derive(Debug, Clone)]
pub struct Robot {
    pub id: u32,
    pub position: Vec2,
    pub angle: f32,
    pub time: time::Instant,
    pub confidence: f32,
    pub tags: HashMap<String, String>, //追加する
}

impl Robot {
    pub fn new(id: u32, position: Vec2, angle: f32, confidence: f32) -> Robot {
        Robot {
            id: id,
            position: position,
            angle: angle,
            time: time::Instant::now(),
            confidence: confidence,
            tags: HashMap::new(),
        }
    }
    pub fn is_alive(&self, limit: time::Duration) -> bool {
        let d = time::Instant::now() - self.time;
        d < limit
    }
    pub fn alive(&mut self) {
        self.time = time::Instant::now();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    pub position: Vec2,
    pub time: time::Instant, //追加する
    pub confidence: f32,
}

impl Ball {
    pub fn new(position: Vec2, confidence: f32) -> Ball {
        Ball {
            position: position,
            time: time::Instant::now(),
            confidence: confidence,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Team {
    pub robots: Vec<Box<Robot>>,
    pub name: String,
    pub score: u32,
    pub red_card: u32,
    pub yellow_card: u32,
    pub goalie: u32,//ゴールキーパ
}

impl Default for Team {
    fn default() -> Team {
        Team {
            robots: vec![],
            name: "unknown".to_owned(),
            score: 0,
            red_card: 0,
            yellow_card: 0,
            goalie: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TeamColor {
    Blue,
    Yellow,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Command {
    Halt,
    Stop,
    NormalStart,
    ForceStart,
    PrepareKickOff(TeamColor),
    PreparePenalty(TeamColor),
    DirectFree(TeamColor),
    IndirectFree(TeamColor),
    Timeout(TeamColor),
    Goal(TeamColor),
    BallPlacement(TeamColor)
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Stage {
    NormalFirstHalfPre,
    NormalFirstHalf,
    NormalHalfTime,
    NormalSecondHalfPre,
    NormalSecondHalf,
    ExtraTimeBreak,
    ExtraFirstHalfPre,
    ExtraFirstHalf,
    ExtraHalfTime,
    ExtraSecondHalfPre,
    ExtraSecondHalf,
    PenaltyShootoutBreak,
    PenaltyShootout,
    PostGame,
}

#[derive(Debug, Clone)]
pub struct World {
    pub balls: Vec<Box<Ball>>,
    pub blues: Team,
    pub yellows: Team,
    pub command: Option<Command>,
    pub stage:Option<Stage>,
    pub timestamp:time::Instant,
}

impl Default for World {
    fn default() -> World {
        World {
            balls: vec![],
            blues: Team::default(),
            yellows: Team::default(),
            command: None,
            stage:None,
            timestamp:time::Instant::now()
        }
    }
}
