use super::grSim_Replacement::{grSim_Replacement,grSim_BallReplacement,grSim_RobotReplacement};
use super::grSim_Packet::grSim_Packet;
use glm::Vec2;

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Team{
    Blue,
    Yellow
}

#[derive(Debug)]
pub struct Robot{
    pub position:Vec2,
    pub angle:f32,
    pub team:Team,
    pub id:u32,
    pub turnon:Option<bool>,
}

impl Robot{
    fn from(g:&grSim_RobotReplacement)->Robot{
        Robot{
            position:Vec2::new(g.get_x() as f32,g.get_y() as f32),
            angle:g.get_dir() as f32,
            team:if g.get_yellowteam(){Team::Yellow}else{Team::Blue},
            id:g.get_id(),
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

//作るだけ作っておいた
#[derive(Debug)]
pub struct Command{
    pub id :u32,
    pub kick_x:f32,
    pub kick_y:f32,
    pub veltangent:f32,
    pub velnormal:f32,
    pub velangular:f32,
    pub spinner:bool,
    pub wheelsspeed:bool,
    pub wheel1:Option<f32>,
    pub wheel2:Option<f32>,
    pub wheel3:Option<f32>,
    pub wheel4:Option<f32>
}
//作るだけ作っておいた
#[derive(Debug)]
pub struct Commands{
    pub timestamp:f64,
    pub team:Team,
    pub commands:Vec<Command>,
}

#[derive(Debug)]
pub struct Packet{
    pub replacement:Option<Replacement>,
    pub commands:Option<Command>,
}

impl Packet{
    pub fn from(g:&grSim_Packet)->Packet{
        Packet{
            commands:None,//この関数は受信に使われるはず
            replacement:if g.has_replacement(){
                Some(Replacement::from(g.get_replacement()))
            }else{
                None
            }
        }
    }
}