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
            ball: None,
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
    pub fn gen_vec2<R: Rng + ?Sized>(&self, ramdom: &mut R) -> Vec2 {
        let normal = Normal::new(0.0 as f32, self.standard_deviation as f32).unwrap();
        vec2(normal.sample(ramdom), normal.sample(ramdom))
    }

    pub fn gen_vec2rad<R: Rng + ?Sized>(&self, ramdom: &mut R) -> Vec2Rad {
        let normal_xy = Normal::new(0.0 as f32, self.standard_deviation as f32).unwrap();
        let normal_theta = Normal::new(0.0 as f32, self.standard_deviation_rad as f32).unwrap();
        vec2rad(
            normal_xy.sample(ramdom),
            normal_xy.sample(ramdom),
            normal_theta.sample(ramdom),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub robots: HashMap<RobotID, Robot>,
    pub ball: Option<Ball>,
}

impl Scene {
    #[allow(dead_code)]
    pub fn new(robots: HashMap<RobotID, Robot>, ball: Option<Ball>) -> Scene {
        Scene {
            robots: robots,
            ball: ball,
        }
    }
    #[allow(dead_code)]
    pub fn noise<R: Rng + ?Sized>(&self, random: &mut R, sn: &SceneNoise) -> Scene {
        let robots: HashMap<RobotID, Robot> = self
            .robots
            .iter()
            .map(|(id, robot): (&RobotID, &Robot)| {
                let noised = robot.position + sn.gen_vec2rad(random);
                (*id, Robot::new(noised, DIAMETOR_ROBOT))
            })
            .collect();
        if let Some(ball) = self.ball {
            let ball = Ball::new(ball.position + sn.gen_vec2(random));
            Scene::new(robots, Some(ball))
        } else {
            Scene::new(robots, None)
        }
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

    pub fn ball_find(&self, era: usize) -> Option<Ball> {
        if era < HISTORY_DEPTH {
            self.scenes[era].ball
        } else {
            None
        }
    }

    pub fn ball_position(&self) -> Option<Vec2> {
        let first = self.ball_find(0)?;
        Some(first.position)
    }

    pub fn ball_velocity(&self) -> Option<Vec2> {
        let first = self.ball_find(0)?;
        let second = self.ball_find(1)?;
        //差分方程式を解く (x0 - x1)/t
        let velocity = (first.position - second.position) / self.period;
        Some(velocity)
    }

    pub fn ball_acceleration(&self) -> Option<Vec2> {
        let first = self.ball_find(0)?;
        let second = self.ball_find(1)?;
        let third = self.ball_find(2)?;
        //差分方程式を解く (x0 - 2x1 + x2)/t^2
        let acceleration =
            (first.position - second.position * 2.0 + third.position) / self.period.powi(2);
        Some(acceleration)
    }

    pub fn ball_jerk(&self) -> Option<Vec2> {
        let first = self.ball_find(0)?;
        let second = self.ball_find(1)?;
        let third = self.ball_find(2)?;
        let forth = self.ball_find(3)?;
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

        let ball = if let Some(ball) = self.now().ball {
            //変数
            let default = vec2(0.0, 0.0);
            let position = ball.position;
            let velocity = self.ball_velocity().unwrap_or(default);
            let acceleration = self.ball_acceleration().unwrap_or(default);
            let jerk = self.ball_jerk().unwrap_or(default);
            let period = self.period;
            //計算 x+vt+1/2*at^2+1/6*yt^3
            let result = position
                + velocity * period
                + acceleration * period.powi(2) / 2.0
                + jerk * period.powi(3) / 6.0;
            Some(Ball::new(result))
        } else {
            None
        };
        Scene::new(robots, ball)
    }
}

trait Shape{
    
}

#[derive(Debug, Clone,Copy)]
pub struct Rectangle{
    top_left:Vec2,//左上
    diagonal:Vec2//対角
}

#[derive(Debug, Clone,Copy)]
pub struct Circle{
    radius:f32,
    center:u32
}

impl Rectangle{
    pub fn new(top_left:Vec2,diagonal:Vec2)->Rectangle{
        Rectangle{
            top_left:top_left,
            diagonal:diagonal
        }
    }
}

impl Circle{
    pub fn new(radius:f32,center:u32)->Circle{
        Circle{
            radius:radius,
            center:center
        }
    }
}

impl Shape for Rectangle{

}

impl Shape for Circle{

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
    pub penalty_area_width: f32,
    pub penalty_area_depth: f32,
}

trait Overlap<T> {
    //重なっている
    fn overlap(&self, rhs: &T) -> bool;
}

impl Overlap<Robot> for Field {
    fn overlap(&self, rhs: &Robot) -> bool {
        let infield = self.infield / 2.0;
        let robot_abs = rhs.position.to_vec2().abs();
        infield.x >= robot_abs.x && infield.y >= robot_abs.y
    }
}

impl Overlap<Ball> for Field {
    fn overlap(&self, rhs: &Ball) -> bool {
        let infield = self.infield / 2.0;
        let ball_abs = rhs.position.abs();
        infield.x >= ball_abs.x && infield.y >= ball_abs.y
    }
}

impl Default for Field {
    fn default() -> Field {
        //適当な値で初期化している[m]
        Field {
            infield: vec2(1.0, 1.0),
            outfield: vec2(1.1, 1.1),
            penalty_area_width: 0.5,
            penalty_area_depth: 0.2,
        }
    }
}

impl Field {
    #[allow(dead_code)]
    pub fn new(
        infield: Vec2,
        outfield: Vec2,
        penalty_area_width: f32,
        penalty_area_depth: f32,
    ) -> Field {
        Field {
            infield: infield,
            outfield: outfield,
            penalty_area_width: penalty_area_width,
            penalty_area_depth: penalty_area_depth,
        }
    }

    #[allow(dead_code)]
    pub fn ramdon_scene<R: Rng + ?Sized>(
        &self,
        random: &mut R,
        blues: u32,
        yellows: u32,
        ball: bool,
    ) -> Scene {
        //Scene::default()

        let random_robot = |r: &mut R| -> Robot {
            Robot::new(
                vec2rad(
                    r.gen_range(-self.infield.x / 2.0, self.infield.x / 2.0),
                    r.gen_range(-self.infield.y / 2.0, self.infield.y / 2.0),
                    r.gen_range(0.0, 2.0 * std::f32::consts::PI),
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
        let ball = if ball {
            Some(random_ball(random))
        } else {
            None
        };
        Scene {
            ball: ball,
            robots: robots,
        }
    }

    //枝刈りメソッド
    #[allow(dead_code)]
    pub fn prune<'a>(&self, scene: &'a Scene) -> Option<&'a Scene> {
        if !scene.robots.values().all(|r: &Robot| self.overlap(r)) {
            return None;
        }if !scene.ball.iter().all(|b: &Ball| self.overlap(b)) {
            return None;
        } else {
            return Some(scene);
        }
    }
}

/*#[cfg(test)]
mod tests {
    use super ::*;
    use super::super::plot::Plotable;
    #[test]
    fn prune() {
        let mut robots = HashMap::new();
        let mut figure = gnuplot::Figure::new();
        let position = Vec2Rad::new(0.51,0.51,0.0);
        robots.insert(RobotID::Blue(1), Robot::new(position,0.1));
        let mut balls = HashMap::new();
        let ballid:BallID = 1;
        balls.insert(ballid,Ball::new(vec2(0.0,0.0)));
        let scene = Scene::new(robots, balls);
        let field = Field::default();
        let scene_prune = field.prune(&scene).unwrap();
        scene_prune.plot(&mut figure);

        std::fs::create_dir_all("img").unwrap();
        figure.save_to_png("img/test_plot.png", 1000, 1000).unwrap();
    }
}*/
