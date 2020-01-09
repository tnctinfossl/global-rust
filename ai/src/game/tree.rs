extern crate model;
extern crate serde;
extern crate serde_derive;
use super::*;
use glm::*;
use rand::*;
use std::collections::HashMap;
use std::rc::Rc;
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
    pub fn simulate<R: Rng + ?Sized>(&self) -> Scene {
        let robots: HashMap<RobotID, Robot> = self
            .now()
            .robots
            .iter()
            .map(|(id, robot): (&RobotID, &Robot)| {
                //変数
                let position = robot.position;
                let velocity = self.robot_velocity(*id).unwrap_or_default();
                let acceleration = self.robot_acceleration(*id).unwrap_or_default();
                let jerk = self.robot_jerk(*id).unwrap_or_default();
                let period = self.period;
                //計算 x+vt+1/2*at^2+1/6*yt^3
                let mut result = position;
                result += velocity * period;
                result += acceleration * period.powi(2) / 2.0;
                result += jerk * period.powi(3) / 6.0;
                (*id, Robot::new(result, robot.diametor))
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
    //途中
    /*#[allow(dead_code)]
    pub Fn evaluation<G:Fn(History)->Scene, E:Fn(Scene)->(f32,f32),C:Fn(Scene)->Option<Scene> >(h:History,g:G,e:E,c:C,d:u32)->{
        
    }*/
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
