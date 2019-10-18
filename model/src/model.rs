use glm::{distance, Vec2};
use std::collections::HashMap;
use std::time::{Duration, Instant};
#[derive(Debug, Clone)]
pub struct Robot {
    pub id: u32,
    pub position: Vec2,
    pub angle: f32,
    pub time: Instant,
    pub confidence: f32,
    pub tags: HashMap<String, String>, //追加する
}

impl Robot {
    pub fn new(id: u32, position: Vec2, angle: f32, confidence: f32) -> Robot {
        Robot {
            id: id,
            position: position,
            angle: angle,
            time: Instant::now(),
            confidence: confidence,
            tags: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    pub position: Vec2,
    pub time: Instant, //追加する
    pub confidence: f32,
}

impl Ball {
    pub fn new(position: Vec2, confidence: f32) -> Ball {
        Ball {
            position: position,
            time: Instant::now(),
            confidence: confidence,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Team {
    pub robots: Vec<Box<Robot>>,
    pub name: Option<String>,
    pub score: Option<u32>,
    pub red_card: Option<u32>,
    pub yellow_card: Option<u32>,
    pub goalie: Option<u32>, //ゴールキーパ
}

impl Default for Team {
    fn default() -> Team {
        Team {
            robots: vec![],
            name: None,
            score: None,
            red_card: None,
            yellow_card: None,
            goalie: None,
        }
    }
}

impl Team {
    pub fn merge(&mut self, newer: Team, now: Instant, options: &MergeOptions) {
        //寿命チェック
        self.robots
            .retain(|robot| (now - robot.time) < options.time_limit);
        //同一性チェックと更新
        for new_robot in newer.robots.into_iter() {
            //同じ場所にロボットは複数台存在できない
            if let Some(old_robot) = self.robots.iter_mut().find(|robot| {
                robot.id == new_robot.id
                    && distance(new_robot.position, robot.position) < options.mergin
            }) {
                //更新
                old_robot.position = new_robot.position;
                old_robot.angle = new_robot.angle;
                old_robot.time = new_robot.time;
                old_robot.confidence = new_robot.confidence;
            }else{
                self.robots.push(new_robot);
            }
        }
        
        if let Some(name)=newer.name{
            self.name=Some(name);
        }
        self.score=newer.score.or(self.score);
        self.red_card=newer.red_card.or(self.red_card);
        self.yellow_card=newer.yellow_card.or(self.yellow_card);
        self.goalie=newer.goalie.or(self.goalie);
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
    BallPlacement(TeamColor),
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
    pub stage: Option<Stage>,
    pub timestamp: Instant,
}

impl Default for World {
    fn default() -> World {
        World {
            balls: vec![],
            blues: Team::default(),
            yellows: Team::default(),
            command: None,
            stage: None,
            timestamp: Instant::now(),
        }
    }
}

pub struct MergeOptions {
    mergin: f32, //同一オブジェクトとみなす距離[mm]
    time_limit: Duration,
}

impl Default for MergeOptions {
    fn default() -> MergeOptions {
        MergeOptions {
            mergin: 10.0,
            time_limit: Duration::from_secs_f32(5.0),
        }
    }
}
#[allow(dead_code)]
impl World {
    pub fn new() -> World {
        World::default()
    }
    pub fn merge(&mut self, newer: World, options: &MergeOptions) {
        //寿命チェック
        let now = newer.timestamp;
        self.balls
            .retain(|ball| (now - ball.time) < options.time_limit);
        //同一性確認と更新
        for new_ball in newer.balls.iter() {
            if let Some(nearest_ball) = self
                .balls
                .iter_mut()
                .find(|old_ball| distance(new_ball.position, old_ball.position) < options.mergin)
            {
                nearest_ball.position = new_ball.position;
                nearest_ball.time = new_ball.time;
            } else {
                self.balls.push(new_ball.clone());
            }
        }
        //teamについて
        self.blues.merge(newer.blues, now, options);
        self.yellows.merge(newer.yellows, now, options);
        //status
        self.command = newer.command.or(self.command);
        self.stage = newer.stage.or(self.stage);
        self.timestamp = now;
    }
}
