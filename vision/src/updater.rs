use super::messages_robocup_ssl_detection::*;
use super::messages_robocup_ssl_wrapper::*;
use glm::{distance, Vec2};
use model;
use std::time::{Duration, Instant};
pub struct Updater {
    mergin: f32, //同一オブジェクトとみなす距離[mm]
    time_limit: Duration,
}

impl Updater {
    pub fn new(mergin: f32, time_limit: Duration) -> Updater {
        Updater {
            mergin: mergin,
            time_limit: time_limit,
        }
    }

    pub fn update(&self, world: &mut model::World, packet: &SSL_WrapperPacket) {
        if packet.has_detection() {
            let detection = packet.get_detection();
            self.update_balls(&mut world.balls, detection.get_balls());
            self.update_team(&mut world.blues.robots, detection.get_robots_blue());
            self.update_team(&mut world.yellows.robots, detection.get_robots_yellow());
        }
    }

    fn update_balls(&self, dest: &mut Vec<Box<model::Ball>>, src: &[SSL_DetectionBall]) {
        //寿命チェック
        let now = Instant::now();
        dest.retain(|b| (now - b.time) < self.time_limit);

        for newer in src.iter() {
            let position = Vec2::new(newer.get_x(), newer.get_y());
            if let Some(nearest) = dest
                .iter_mut()
                .find(|cmp| distance(position, cmp.position) < self.mergin)
            {
                nearest.position = position;
                nearest.time = now;
            } else {
                dest.push(Box::new(model::Ball::new(position, newer.get_confidence())));
            }
        }
    }

    fn update_team(&self, dest: &mut Vec<Box<model::Robot>>, src: &[SSL_DetectionRobot]) {
        //寿命チェック
        let now = Instant::now();
        dest.retain(|b| (now - b.time) < self.time_limit);
        for newer in src.iter() {
            let id = newer.get_robot_id();
            let position = Vec2::new(newer.get_x(), newer.get_y());
            let angle = if newer.has_orientation() {
                newer.get_orientation()
            } else {
                0.0
            };
            let confidence = newer.get_confidence();
            if let Some(older) = dest.iter_mut().find(|b| b.id == id) {
                if distance(position, older.position) < self.mergin {
                    older.position = position;
                    older.angle = angle;
                    older.time = now;
                    older.confidence = confidence;
                    continue;
                }
            }
            dest.push(Box::new(model::Robot::new(id, position, angle, confidence)));
        }
    }

}
