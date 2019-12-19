extern crate model;
extern crate serde;
extern crate serde_derive;
use super::vec2rad::*;
use glm::*;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use serde_derive::*;
//use std::cell::RefCell;
//use std::borrow::*;
use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::ops::Not;
use std::rc::Rc;

static DIAMETOR_ROBOT: f32 = 0.10; //[m]

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq, Ord, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Robot {
    pub position: Vec2Rad,
    pub diametor: f32,
}

impl Robot {
    #[allow(dead_code)]
    pub fn new(position: Vec2Rad, diametor: f32) -> Robot {
        Robot { position, diametor }
    }
}

type BallID = u32;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Ball {
    pub position: Vec2,
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

impl Default for Scene {
    #[allow(dead_code)]
    fn default() -> Scene {
        Scene {
            robots: HashMap::new(),
            balls: HashMap::new(),
        }
    }
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SceneNoise {
    standard_deviation: f32,     //標準偏差[m]
    standard_deviation_rad: f32, //標準偏差[rad]
}

impl Default for SceneNoise {
    fn default() -> SceneNoise {
        SceneNoise {
            standard_deviation: 0.01, //[m]
            standard_deviation_rad: std::f32::consts::PI,
        }
    }
}

impl SceneNoise {
    #[allow(dead_code)]
    pub fn new(standard_deviation: f32, standard_deviation_rad: f32) -> SceneNoise {
        SceneNoise {
            standard_deviation: standard_deviation,
            standard_deviation_rad: standard_deviation_rad,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub robots: HashMap<RobotID, Robot>,
    pub balls: HashMap<BallID, Ball>,
}

impl Scene {
    #[allow(dead_code)]
    pub fn new(robots: HashMap<RobotID, Robot>, balls: HashMap<BallID, Ball>) -> Scene {
        Scene {
            robots: robots,
            balls: balls,
        }
    }
    #[allow(dead_code)]
    pub fn noise<R: Rng + ?Sized>(&self, random: &mut R, sn: &SceneNoise) -> Scene {
        let robots: HashMap<RobotID, Robot> = self
            .robots
            .iter()
            .map(|(id, robot): (&RobotID, &Robot)| {
                let mut noised = robot.position;
                let normal_x = Normal::new(noised.x as f64, sn.standard_deviation as f64).unwrap();
                let normal_y = Normal::new(noised.y as f64, sn.standard_deviation as f64).unwrap();
                let normal_theta =
                    Normal::new(noised.theta as f64, sn.standard_deviation_rad as f64).unwrap();
                noised.x += normal_x.sample(random) as f32;
                noised.y += normal_y.sample(random) as f32;
                noised.theta += normal_theta.sample(random) as f32;
                (*id, Robot::new(noised, DIAMETOR_ROBOT))
            })
            .collect();
        let balls: HashMap<BallID, Ball> = self
            .balls
            .iter()
            .map(|(id, ball): (&BallID, &Ball)| {
                let mut noised = ball.position;
                let normal_x = Normal::new(noised.x as f64, sn.standard_deviation as f64).unwrap();
                let normal_y = Normal::new(noised.y as f64, sn.standard_deviation as f64).unwrap();
                noised.x += normal_x.sample(random) as f32;
                noised.y += normal_y.sample(random) as f32;
                (*id, Ball::new(noised))
            })
            .collect();
        Scene::new(robots, balls)
    }
}
const HISTORY_DEPTH: usize = 4;

#[derive(Debug, Clone)]
pub struct History {
    pub period: f32, //非ゼロかつ正の値を保証すること
    pub scenes: [Rc<Scene>; HISTORY_DEPTH],
}

impl History {
    #[allow(dead_code)]
    pub fn new(period: f32, scenes: [Rc<Scene>; HISTORY_DEPTH]) -> History {
        if period <= 0.0 {
            panic!();
        } else {
            History {
                period: period,
                scenes: scenes,
            }
        }
    }
    #[inline(always)]
    pub fn now<'a>(&'a self) -> &'a Scene {
        &self.scenes[0]
    }

    pub fn robot_find(&self, era: usize, id: RobotID) -> Option<Robot> {
        if era < HISTORY_DEPTH {
            if let Some(&r) = self.scenes[era].robots.get(&id) {
                Some(r)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn robot_position(&self, id: RobotID) -> Option<Vec2Rad> {
        if let Some(r) = self.robot_find(0, id) {
            Some(r.position)
        } else {
            None
        }
    }

    pub fn robot_velocity(&self, id: RobotID) -> Option<Vec2Rad> {
        //データの取得//use super::*;
        let now = self.robot_find(0, id)?;
        let last = self.robot_find(1, id)?;
        //計算する
        Some(Vec2Rad::diff(self.period, now.position, last.position))
    }

    pub fn robot_acceleration(&self, id: RobotID) -> Option<Vec2Rad> {
        //データの取得
        let first = self.robot_find(0, id)?;
        let second = self.robot_find(1, id)?;
        let third = self.robot_find(2, id)?;
        //差分方程式を計算する
        Some(Vec2Rad::diff3(
            self.period,
            first.position,
            second.position,
            third.position,
        ))
    }

    pub fn robot_jerk(&self, id: RobotID) -> Option<Vec2Rad> {
        //データの取得
        let first = self.robot_find(0, id)?;
        let second = self.robot_find(1, id)?;
        let third = self.robot_find(2, id)?;
        let forth = self.robot_find(3, id)?;
        //差分方程式を解く
        Some(Vec2Rad::diff4(
            self.period,
            first.position,
            second.position,
            third.position,
            forth.position,
        ))
    }

    pub fn ball_find(&self, era: usize, id: BallID) -> Option<Ball> {
        if era < HISTORY_DEPTH {
            if let Some(&ball) = self.scenes[era].balls.get(&id) {
                Some(ball)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn ball_position(&self, id: BallID) -> Option<Vec2> {
        let first = self.ball_find(0, id)?;
        Some(first.position)
    }

    pub fn ball_velocity(&self, id: BallID) -> Option<Vec2> {
        let first = self.ball_find(0, id)?;
        let second = self.ball_find(1, id)?;
        //差分方程式を解く (x0 - x1)/t
        let velocity = (first.position - second.position) / self.period;
        Some(velocity)
    }

    pub fn ball_acceleration(&self, id: BallID) -> Option<Vec2> {
        let first = self.ball_find(0, id)?;
        let second = self.ball_find(1, id)?;
        let third = self.ball_find(2, id)?;
        //差分方程式を解く (x0 - 2x1 + x2)/t^2
        let acceleration =
            (first.position - second.position * 2.0 + third.position) / self.period.powi(2);
        Some(acceleration)
    }

    pub fn ball_jerk(&self, id: BallID) -> Option<Vec2> {
        let first = self.ball_find(0, id)?;
        let second = self.ball_find(1, id)?;
        let third = self.ball_find(2, id)?;
        let forth = self.ball_find(3, id)?;
        //差分方程式を解く (x0 - 3x1 + 3x2 - x3)/t^3
        let jerk = (first.position - second.position * 3.0 + third.position * 3.0 - forth.position)
            / self.period.powi(3);
        Some(jerk)
    }

    //x+vt+1/2*at^2+1/6*yt^3を求めることで次のシーンを予想する
    #[allow(dead_code)]
    pub fn simulate<R: Rng + ?Sized>(
        &self,
        _size: usize,
        _random: &mut R,
        _field: &Field,
    ) -> Scene {
        let robots: HashMap<RobotID, Robot> = self
            .now()
            .robots
            .keys()
            .flat_map(|id: &RobotID| {
                //変数
                let position = self.robot_position(*id)?;
                let velocity = self.robot_velocity(*id)?;
                let acceleration = self.robot_acceleration(*id)?;
                let jerk = self.robot_jerk(*id)?;
                let period = self.period;
                //計算 x+vt+1/2*at^2+1/6*yt^3
                let mut result = position;
                result += velocity * period;
                result += acceleration * period.powi(2) / 2.0;
                result += jerk * period.powi(3) / 6.0;
                Some((*id, Robot::new(result, DIAMETOR_ROBOT)))
            })
            .collect();
        let balls: HashMap<BallID, Ball> = self
            .now()
            .balls
            .keys()
            .flat_map(|id: &BallID| {
                //変数
                let position = self.ball_position(*id)?;
                let velocity = self.ball_velocity(*id)?;
                let acceleration = self.ball_acceleration(*id)?;
                let jerk = self.ball_jerk(*id)?;
                let period = self.period;
                //計算 x+vt+1/2*at^2+1/6*yt^3
                let result = position
                    + velocity * period
                    + acceleration * period.powi(2) / 2.0
                    + jerk * period.powi(3) / 6.0;
                Some((*id, Ball::new(result)))
            })
            .collect();
        Scene::new(robots, balls)
    }
}

#[derive(Debug, Clone)]
pub struct TreeBuilder {
    pub max_node: u32,
    pub max_depth: u32,
}

impl TreeBuilder {
    #[allow(dead_code)]
    pub fn new(parent_history: &History) -> Tree {
        let parent = parent_history.clone();
        let children = Vec::new();
        Tree {
            parent: parent,
            children: children,
            score: (0.0, 0.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tree {
    pub parent: History,
    pub children: Vec<History>,
    pub score: (f32, f32),
}

impl Tree {
    /*#[allow(dead_code)]
    pub fn new(parent_history: &History) -> Tree {
        let parent = parent_history.clone();
        let children = Vec::new();
        Tree {
            parent: parent,
            children: children,
            score: (0.0, 0.0),
        }
    }*/

    #[allow(dead_code)]
    pub fn new_children(&self, number: u32) -> Tree {
        let parent = self.parent.clone();
        let score = self.score;
        let mut children = self.children.clone();
        let scenenoise = SceneNoise::default();
        let tmp = &self.parent.scenes;
        let target = &self.parent.scenes[0].clone();
        let mut num = number;

        loop {
            children.push(History::new(
                1.0,
                [
                    Rc::new(target.noise(&mut rand::thread_rng(), &scenenoise)),
                    tmp[0].clone(),
                    tmp[1].clone(),
                    tmp[2].clone(),
                ],
            ));
            num = num - 1;
            if num <= 0 {
                break;
            }
        }
        Tree {
            parent: parent,
            children: children,
            score: score,
        }
    }
    //ジェネリクスでかく
    //pub fn evaluation<T>(fn:T)->{}
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Field {
    pub infield: Vec2,
    pub outfield: Vec2,
}

trait Contain<T> {
    fn contain(&self, rhs: &T) -> bool;
}

impl Contain<Robot> for Field {
    fn contain(&self, _rhs: &Robot) -> bool {
        let infield = self.infield;
        let robot_abs = _rhs.position.to_vec2().abs();
        if infield.x / 2.0 >= robot_abs.x && infield.y / 2.0 >= robot_abs.y {
            true
        } else {
            false
        }
    }
}

impl Contain<Ball> for Field {
    fn contain(&self, _bhs: &Ball) -> bool {
        let infield = self.infield;
        let ball_abs = _bhs.position.abs();
        if infield.x / 2.0 >= ball_abs.x && infield.y / 2.0 >= ball_abs.y {
            true
        } else {
            false
        }
    }
}

impl Default for Field {
    fn default() -> Field {
        //適当な値で初期化している
        Field {
            infield: vec2(1.0, 1.0),
            outfield: vec2(1.1, 1.1),
        }
    }
}

impl Field {
    #[allow(dead_code)]
    pub fn new(infield: Vec2, outfield: Vec2) -> Field {
        Field {
            infield: infield,
            outfield: outfield,
        }
    }

    #[allow(dead_code)]
    pub fn ramdon_scene<R: Rng + ?Sized>(
        &self,
        random: &mut R,
        blues: u32,
        yellows: u32,
        balls: u32,
    ) -> Scene {
        //Scene::default()

        let random_robot = |r: &mut R| -> Robot {
            Robot::new(
                vec2rad(
                    r.gen_range(-self.infield.x / 2.0, self.infield.x / 2.0),
                    r.gen_range(-self.infield.y / 2.0, self.infield.y / 2.0),
                    r.gen_range(0.0, 2.0 * std::f32::consts::PI)
                ),
                DIAMETOR_ROBOT,
            )
        };

        let random_ball = |r: &mut R| -> Ball {
            Ball::new(vec2(
                r.gen_range(-self.infield.x / 2.0, self.infield.x / 2.0),
                r.gen_range(-self.infield.y / 2.0, self.infield.y / 2.0),
            ))
        };

        let robots = (0..blues)
            .map(|id| RobotID::Blue(id))
            .chain((0..yellows).map(|id| RobotID::Yellow(id)))
            .map(|id| (id, random_robot(random)))
            .collect();

        let balls: HashMap<BallID, Ball> = (0..balls).map(|id| (id, random_ball(random))).collect();
        Scene {
            balls: balls,
            robots: robots,
        }
    }


    //枝刈りメソッド
    #[allow(dead_code)]
    pub fn prune<'a>(&self, scene: &'a Scene) -> Option<&'a Scene> {
        let jodge_robots = scene
        .robots
        .values()
        //.map(|r: &Robot| self.check_robots_position(r.position))
        .map(|r: &Robot|self.contain(r))
        .find(|x| *x == false);
        let unwrap_robots = 
            match jodge_robots{//範囲外があったらtrue
            None => false,
            Some(i) => i
            };
        
        let jodge_balls = scene
            .balls
            .values()
            //.map(|b: &Ball| self.check_balls_position(b.position))
            .map(|b:&Ball|self.contain(b))
            .find(|x| *x == false);
        let unwrap_balls = 
            match jodge_balls{//範囲外があったらtrue
            None => false,
            Some(i) => i
        };
        if unwrap_robots || unwrap_balls {//どちらかに範囲外があるとき
            Some(scene)
        } else {
            None
        }
    }
}


/*#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prune() {
        let scene = Scene::default();

    }
}*/
