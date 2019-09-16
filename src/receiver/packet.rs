use super::grSim_Replacement::{grSim_Replacement,grSim_BallReplacement,grSim_RobotReplacement};
use glm::Vec2;

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum RobotID{
    Yellow(u32),
    Blue(u32)
}

#[derive(Debug)]
pub struct Robot{
    pub position:Vec2,
    pub angle:f32,
    pub id:RobotID,
    pub turnon:Option<bool>,
}

impl Robot{
    fn from(g:&grSim_RobotReplacement)->Robot{
        Robot{
            position:Vec2::new(g.get_x() as f32,g.get_y() as f32),
            angle:g.get_dir() as f32,
            id:if g.get_yellowteam(){
                RobotID::Yellow(g.get_id())
            }else{
                RobotID::Blue(g.get_id())
            },
            turnon:if g.has_turnon(){
                Some(g.get_turnon())
            }else{
                None
            }
        }
    }
}

#[derive(Debug)]
pub struct Ball{
    pub position:Vec2,
    pub velocity:Vec2,
}

impl Ball{
    fn from(g:&grSim_BallReplacement)->Ball{
        Ball{
            position:Vec2::new(g.get_x() as f32,g.get_y() as f32),
            velocity:Vec2::new(g.get_vx()as f32,g.get_vy() as f32)
        }
    }
}

//TODO あとで適切な場所に移動する
#[derive(Debug)]
pub struct Replacement{
    pub ball:Option<Ball>,
    pub robots:Vec<Robot>
}

impl Replacement{
    pub fn from(g:&grSim_Replacement)->Replacement{
        Replacement{
            ball:if g.has_ball(){
                Some(Ball::from(g.get_ball()))
            }else{
                None
            },
            robots:g.get_robots().iter().map(|x|Robot::from(x)).collect()
        }
    }
}
