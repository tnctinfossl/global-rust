use super::messages_robocup_ssl_wrapper::*;
use super::messages_robocup_ssl_detection::*;
use model;
use std::time::{Duration,Instant};
use glm::{Vec2,distance};
pub struct Updater{
    mergin :f32,//同一オブジェクトとみなす距離[mm]
    time_limit:Duration,
}

impl Updater{
    pub fn new(mergin:f32,time_limit:Duration)->Updater{
        Updater{
            mergin:mergin,
            time_limit:time_limit
        }
    }

    pub fn update(&self, world:&mut model::World,packet: &SSL_WrapperPacket){
        if packet.has_detection(){
            let detection = packet.get_detection();
            self.update_balls(&mut world.balls,detection.get_balls());
            self.update_blues(&mut world.blues,detection.get_robots_blue());
            self.update_yellows(&mut world.yellows,detection.get_robots_yellow());
        }
    }

    fn update_balls(&self, dest:&mut Vec<Box<model::Ball>>,src:&[SSL_DetectionBall]){
        //寿命チェック
        let now = Instant::now();
        dest.retain(|b|(now-b.time)<self.time_limit);
        
        for newer in src.iter(){
            let position = Vec2::new(newer.get_x(),newer.get_y());
            if let Some(nearest)=dest.iter_mut().find(|cmp|distance(position,cmp.position)<self.mergin){
                nearest.position=position;
                nearest.time=now;
            }else{
                dest.push(Box::new(model::Ball::new(position,newer.get_confidence())));
            }
        }
    }

    fn update_blues(&self,dest:&mut Vec<Box<model::Robot>>,src:&[SSL_DetectionRobot]){
        //寿命チェック
        let now = Instant::now();
        dest.retain(|b|(now-b.time)<self.time_limit);
        for newer in src.iter(){
            let id = newer.get_robot_id();
            let position = Vec2::new(newer.get_x(),newer.get_y());
            let angle = if newer.has_orientation(){
                newer.get_orientation()
            }else{
                0.0
            };
            let confidence=newer.get_confidence();
            if let Some(older)=dest.iter_mut().find(|b|b.id==id){       
                if distance(position,older.position)<self.mergin{
                    older.position=position;
                    older.angle=angle;
                    older.time=now;
                    older.confidence=confidence;
                    continue;
                }
            }
            dest.push(model::Robot::new(id,position,angle,confidence));
        }
    }
    fn update_yellows(&self,dest:&mut Vec<Box<model::Robot>>,src:&[SSL_DetectionRobot]){
        //寿命チェック
                //寿命チェック
        let now = Instant::now();
        dest.retain(|b|(now-b.time)<self.time_limit);
        for newer in src.iter(){
            let id = newer.get_robot_id();
            let position = Vec2::new(newer.get_x(),newer.get_y());
            let angle = if newer.has_orientation(){
                newer.get_orientation()
            }else{
                0.0
            };
            let confidence=newer.get_confidence();
            if let Some(older)=dest.iter_mut().find(|b|b.id==id){       
                if distance(position,older.position)<self.mergin{
                    older.position=position;
                    older.angle=angle;
                    older.time=now;
                    older.confidence=confidence;
                    continue;
                }
            }
            dest.push(model::Robot::new(id,position,angle,confidence));
        }
    }

}



/*-
trait Updater{
    type Input;
    fn update(&mut self,data:&Self::Input);
}

impl Updater for model::World{
    type Input=SSL_WrapperPacket;
    fn update(&mut self,wrapper_packet:&Self::Input){
        if wrapper_packet.has_detection(){
            let detection=wrapper_packet.get_detection();
            self.balls.update(detection.get_balls());
        }
        //self.balls.update(&frame.get_ball());
    }
}

*/
/*

impl Updater for Vec<model::Robot>{
    type Input=&[SSL_DetectionBall];
    fn update(&mut self,data:&Input){

    }
}
*/

/*
impl World {
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
*/