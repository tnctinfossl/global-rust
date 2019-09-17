use super::messages_robocup_ssl_detection::{
    SSL_DetectionBall, SSL_DetectionFrame, SSL_DetectionRobot,
};
//use super::messages_robocup_ssl_geometry::
use super::messages_robocup_ssl_wrapper::SSL_WrapperPacket;
use glm::Vec2;
#[derive(Debug)]
pub struct Ball {
    pub confidence: f32,
    pub position: Vec2,
}

impl Ball {
    pub fn from_message(b: &SSL_DetectionBall) -> Ball {
        Ball {
            confidence: b.get_confidence(),
            position: Vec2::new(b.get_x(), b.get_y()),
        }
    }
}

#[derive(Debug)]
pub struct Robot {
    pub confidence: f32,
    pub id: u32,
    pub position: Vec2,
    pub angle: f32,
}

impl Robot {
    pub fn from_message(r: &SSL_DetectionRobot) -> Robot {
        Robot {
            confidence: r.get_confidence(),
            id: r.get_robot_id(),
            position: Vec2::new(r.get_x(), r.get_y()),
            angle: r.get_orientation(),
        }
    }
}

#[derive(Debug)]
pub struct World {
    pub balls: Vec<Ball>,
    pub yellows: Vec<Robot>,
    pub blues: Vec<Robot>,
}

impl World {
    pub fn from_message(w: &SSL_WrapperPacket) -> Option<World> {
        if w.has_detection() {
            let d = w.get_detection();
            Some(World {
                balls: d.get_balls().iter().map(Ball::from_message).collect(),
                yellows: d
                    .get_robots_yellow()
                    .iter()
                    .map(Robot::from_message)
                    .collect(),
                blues: d
                    .get_robots_blue()
                    .iter()
                    .map(Robot::from_message)
                    .collect(),
            })
        } else {
            None
        }
    }
}
