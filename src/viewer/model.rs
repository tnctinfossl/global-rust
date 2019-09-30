use crate::listener;
use glm::{distance, Vec2};
use std::time;
#[derive(Debug)]
pub struct Robot {
    pub id: u32,
    pub position: Vec2,
    pub angle: f32,
    pub time: time::Instant, //追加する
}

impl Robot {
    pub fn new(id:u32,position: Vec2,angle:f32) ->Robot {
        Robot {
            id:id,
            position: position,
            angle:angle,
            time: time::Instant::now(),
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

#[derive(Debug)]
pub struct Ball {
    pub position: Vec2,
    pub time: time::Instant, //追加する
}

impl Ball {
    pub fn new(position: Vec2) -> Ball {
        Ball {
            position: position,
            time: time::Instant::now(),
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

#[derive(Debug)]
pub struct Items {
    pub balls: Vec<Ball>,
    pub blues: Vec<Robot>,
    pub yellows: Vec<Robot>,
}

impl Default for Items {
    fn default() -> Items {
        Items {
            balls: vec![],
            blues: vec![],
            yellows: vec![],
        }
    }
}

impl Items {
    pub fn update(&mut self, w: &listener::World) {
        self.update_balls(&w.balls);
        self.update_blues(&w.blues);
        self.update_yellows(&w.yellows);
    }

    fn update_balls(&mut self, balls: &Vec<listener::Ball>) {
        let merge_distance = 100.0; //mergeする範囲
        let time_limit = time::Duration::from_secs(3); //寿命

        self.balls.retain(move |b| b.is_alive(time_limit));
        for position in balls.iter().map(|b| b.position) {
            if let Some(older) = self
                .balls
                .iter_mut()
                .find(|older| distance(position, older.position) < merge_distance)
            {
                older.position = position;
                older.alive();
            } else {
                self.balls.push(Ball::new(position))
            }
        }
    }

    fn update_blues(&mut self, robots: &Vec<listener::Robot>) {
        let time_limit = time::Duration::from_secs(3); //寿命
        //idをもとに結合する
        self.blues.retain(move|r|r.is_alive(time_limit));
        for newer in robots.iter(){
            if let Some(older)=self.blues.iter_mut().find(|b|b.id==newer.id){
                older.position=newer.position;
                older.angle=newer.angle;
                older.alive();
            }else{
                self.blues.push(Robot::new(newer.id, newer.position, newer.angle));
            }
        }
    }
    fn update_yellows(&mut self, robots: &Vec<listener::Robot>) {
        let time_limit = time::Duration::from_secs(3); //寿命
        //idをもとに結合する
        self.blues.retain(move|r|r.is_alive(time_limit));
        for newer in robots.iter(){
            if let Some(older)=self.yellows.iter_mut().find(|b|b.id==newer.id){
                older.position=newer.position;
                older.angle=newer.angle;
                older.alive();
            }else{
                self.yellows.push(Robot::new(newer.id, newer.position, newer.angle));
            }
        }
    }
}
