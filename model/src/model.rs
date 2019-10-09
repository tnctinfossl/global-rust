use glm::Vec2;
use std::time;
#[derive(Debug,Clone,Copy)]
pub struct Robot {
    pub id: u32,
    pub position: Vec2,
    pub angle: f32,
    pub time: time::Instant, //追加する
    pub confidence:f32,
}

impl Robot {
    pub fn new(id:u32,position: Vec2,angle:f32,confidence:f32) ->Box<Robot> {
        Box::new(Robot {
            id:id,
            position: position,
            angle:angle,
            time: time::Instant::now(),
            confidence:confidence
        })
    }
    pub fn is_alive(&self, limit: time::Duration) -> bool {
        let d = time::Instant::now() - self.time;
        d < limit
    }
    pub fn alive(&mut self){
        self.time=time::Instant::now();
    }
}

#[derive(Debug,Clone,Copy)]
pub struct Ball {
    pub position: Vec2,
    pub time: time::Instant, //追加する
    pub confidence:f32,
}

impl Ball {
    pub fn new(position: Vec2,confidence:f32) -> Ball {
        Ball {
            position: position,
            time: time::Instant::now(),
            confidence:confidence
        }
    }
    pub fn is_alive(&self, limit: time::Duration) -> bool {
        let d = time::Instant::now() - self.time;
        d < limit
    }
    pub fn alive(&mut self){
        self.time=time::Instant::now();
    }
}

#[derive(Debug,Clone)]
pub struct World {
    pub balls: Vec<Box<Ball>>,
    pub blues: Vec<Box<Robot>>,
    pub yellows: Vec<Box<Robot>>,
}

impl Default for World {
    fn default() -> World {
        World {
            balls: vec![],
            blues: vec![],
            yellows: vec![],
        }
    }
}