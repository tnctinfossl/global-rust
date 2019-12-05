use glm::*;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::ops::*;
use std::rc::Rc;
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq, Ord)]
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

#[derive(Debug, Clone, Copy)]
pub struct Robot {
    position: Vec2,
    angle: f32, //rad
}

impl Default for Robot {
    #[allow(dead_code)]
    fn default() -> Robot {
        Robot {
            position: vec2(0.0, 0.0),
            angle: 0.0,
        }
    }
}

impl Robot {
    #[allow(dead_code)]
    fn new(id: RobotID, position: Vec2, angle: f32) -> Robot {
        Robot {
            position: position,
            angle: angle,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    position: Vec2,
}

impl Default for Ball {
    #[allow(dead_code)]
    fn default() -> Ball {
        Ball {
            position: vec2(0.0, 0.0),
        }
    }
}

impl Ball {
    #[allow(dead_code)]
    pub fn new(position: Vec2) -> Ball {
        Ball { position: position }
    }
}

#[derive(Debug, Clone)]
pub struct Scene {
    robots: HashMap<RobotID, Robot>,
    balls: Vec<Ball>,
}

impl Default for Scene {
    #[allow(dead_code)]
    fn default() -> Scene {
        Scene {
            robots: HashMap::new(),
            balls: vec![],
        }
    }
}

impl Scene {
    pub fn new(robots: HashMap<RobotID, RobotID>, balls: Vec<Ball>) -> Scene {
        Scene {
            robots: HashMap::new(),
            balls: balls,
        }
    }
}
const HistoryDepth: usize = 4;

#[derive(Debug, Clone)]
pub struct History {
    elapsed_time: f32, //非ゼロ
    scenes: [Rc<RefCell<Scene>>; HistoryDepth],
}

impl History {
    #[allow(dead_code)]
    pub fn new(period: f32, scenes: [Rc<RefCell<Scene>>; 4]) -> Option<History> {
        if period <= 0.0 {
            None
        } else {
            Some(History {
                elapsed_time: period,
                scenes: scenes,
            })
        }
    }

    #[allow(dead_code)]
    pub fn find(&self, era: usize, id: RobotID) -> Option<Robot> {
        if era < HistoryDepth {
            if let Some(&r) = self.scenes[era].borrow().robots.get(&id) {
                Some(r)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn position(&self,id:RobotID)->Option<(Vec2,f32)>{
        if let Some(r)=self.find(0,id){
            Some((r.position, r.angle))
        }else{
            None
        }
    }
}

/*pub fn position(&self, id: RobotID) -> Option<(Vec2, f32)> {
    use RobotID::*;

    if id <= 0 {
        None
    } else {
        (robot.position, robot.angle)
    }
}*/

/*pub fn velocity(id: RobotID) -> Option<(Vec2, f32)> {}*/

/*pub fn acceleration(id: RobotID) -> Option<(Vec2, f32)> {}*/

/*pub fn jerk(id: RobotID) -> Option<(Vec2, f32)> {}*/

#[derive(Debug, Clone)]
pub struct Future {
    history: History,
}

impl Future {
    /*pub fn new(history: History) -> Scene {
        let history = history;
    }*/
}

#[derive(Debug, Clone)]
pub struct Tree {
    histories: History,
    score: (f32, f32),
}
