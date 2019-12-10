extern crate model;
extern crate serde;
extern crate serde_derive;
use glm::*;
use rand::Rng;
use serde_derive::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Not;
use std::rc::Rc;
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Robot {
    pub position: Vec2,
    pub angle: f32, //rad
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
    fn new(position: Vec2, angle: f32) -> Robot {
        Robot {
            position: position,
            angle: angle,
        }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub robots: HashMap<RobotID, Robot>,
    pub balls: Vec<Ball>,
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
    #[allow(dead_code)]
    pub fn new(robots: HashMap<RobotID, Robot>, balls: Vec<Ball>) -> Scene {
        Scene {
            robots: robots,
            balls: balls,
        }
    }
}
const HISTORY_DEPTH: usize = 4;

#[derive(Debug, Clone)]
pub struct History {
    pub period: f32, //非ゼロ
    pub scenes: [Rc<RefCell<Scene>>; HISTORY_DEPTH],
}

impl History {
    #[allow(dead_code)]
    pub fn new(period: f32, scenes: [Rc<RefCell<Scene>>; 4]) -> Option<History> {
        if period <= 0.0 {
            None
        } else {
            Some(History {
                period: period,
                scenes: scenes,
            })
        }
    }

    #[allow(dead_code)]
    pub fn find(&self, era: usize, id: RobotID) -> Option<Robot> {
        if era < HISTORY_DEPTH {
            if let Some(&r) = self.scenes[era].borrow().robots.get(&id) {
                Some(r)
            } else {
                None
            }
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn position(&self, id: RobotID) -> Option<(Vec2, f32)> {
        if let Some(r) = self.find(0, id) {
            Some((r.position, r.angle))
        } else {
            None
        }
    }
    //[0..2PI]の範囲に入れる
    fn rad_range(x: f32) -> f32 {
        let two_pi = 2.0 * std::f32::consts::PI;
        if x < 0.0 {
            (x % two_pi) + two_pi
        } else {
            x % two_pi
        }
    }

    #[inline(always)]
    fn rad_diff(now: f32, last: f32) -> f32 {
        let pi = std::f32::consts::PI;
        let two_pi = 2.0 * std::f32::consts::PI;
        //正規化する[0..2PI]
        let now = Self::rad_range(now);
        let last = Self::rad_range(last);
        //短い経路を選択する
        if abs(now - last) < (pi + std::f32::EPSILON) {
            now - last
        } else {
            if now > last {
                now - two_pi - last
            } else {
                now + two_pi - last
            }
        }
    }
    //[x_0 - 2*x_1 + x_2]を求める
    #[inline(always)]
    fn rad_diff3(first: f32, second: f32, third: f32) -> f32 {
        //短い経路を求めつつ，[x_0 - 2*x_1 + x_2]を求める
        Self::rad_diff(first, second) - Self::rad_diff(second, third)
    }

    #[inline(always)]
    fn rad_diff4(first: f32, second: f32, third: f32, forth: f32) -> f32 {
        //短い経路を求めつつ，[x_0 - 3*x_1 + 3*x_2 - x_3]を求める
        Self::rad_diff3(first, second, third) - Self::rad_diff3(second, third, forth)
    }

    #[allow(dead_code)]
    pub fn velocity(&self, id: RobotID) -> Option<(Vec2, f32)> {
        //データの取得//use super::*;
        let now = self.find(0, id)?;
        let last = self.find(1, id)?;
        //計算する
        let speed = (now.position - last.position) / self.period;
        let omega = Self::rad_diff(now.angle, last.angle) / self.period;
        Some((speed, omega))
    }

    #[allow(dead_code)]
    pub fn acceleration(&self, id: RobotID) -> Option<(Vec2, f32)> {
        //データの取得
        let first = self.find(0, id)?;
        let second = self.find(1, id)?;
        let third = self.find(2, id)?;
        //差分方程式を計算する
        let acc = (first.position - second.position * 2.0 + third.position) / (self.period.powi(2));
        let alpha = Self::rad_diff3(first.angle, second.angle, third.angle) / (self.period.powi(2));
        Some((acc, alpha))
    }

    #[allow(dead_code)]
    pub fn jerk(&self, id: RobotID) -> Option<(Vec2, f32)> {
        //データの取得
        let first = self.find(0, id)?;
        let second = self.find(1, id)?;
        let third = self.find(2, id)?;
        let forth = self.find(3, id)?;
        //差分方程式を解く
        let jerk = (first.position - second.position * 3.0 + third.position * 3.0 - forth.position)
            / (self.period.powi(3));
        let jerk_rad = Self::rad_diff4(first.angle, second.angle, third.angle, forth.angle)
            / (self.period.powi(3));
        Some((jerk, jerk_rad))
    }
}

#[derive(Debug, Clone)]
pub struct Tree {
    pub children: History,
    pub score: (f32, f32),
}

impl Tree {
    /*
    pub fn new(children: History,score: (f32,f32)) -> Scene{
        let history = children;
    }*/
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Field {
    pub infield: Vec2,
    pub outfield: Vec2,
}

impl Default for Field {
    fn default() -> Field {
        //適当な値で初期化している
        Field {
            infield: vec2(10000.0, 10000.0),
            outfield: vec2(11000.0, 11000.0),
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
                vec2(
                    r.gen_range(-self.infield.x / 2.0, self.infield.x / 2.0),
                    r.gen_range(-self.infield.y / 2.0, self.infield.y / 2.0),
                ),
                r.gen_range(0.0, 2.0 * std::f32::consts::PI),
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

        let balls = (0..balls).map(|_| random_ball(random)).collect();
        Scene {
            balls: balls,
            robots: robots,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eq(a: f32, b: f32) -> bool {
        abs(a - b) < std::f32::EPSILON * 100.0
    }

    #[test]
    fn test_rad_range() {
        let pi = std::f32::consts::PI;
        assert!(eq(History::rad_range(0.0), 0.0));
        assert!(eq(History::rad_range(pi), pi));
        assert!(eq(History::rad_range(3.0 * pi), pi));
        assert!(eq(History::rad_range(-pi), pi));
        assert!(eq(History::rad_range(-3.0 * pi), pi));
    }

    #[test] //cargo test
    fn test_rad_diff() {
        //let two_pi = 2.0 * std::f32::consts::PI;
        let pi = std::f32::consts::PI;
        let pi_2 = std::f32::consts::PI / 2.0;
        //let pi_4 = std::f32::consts::PI / 4.0;

        assert!(eq(History::rad_diff(0.0, 0.0), 0.0));
        assert!(eq(History::rad_diff(pi_2, 0.0), pi_2));
        assert!(eq(History::rad_diff(0.0, pi_2), -pi_2));
        assert!(eq(History::rad_diff(pi + pi_2, 0.0), -pi_2));
        assert!(eq(History::rad_diff(0.0, pi + pi_2), pi_2));
    }
}