extern crate model;
extern crate serde;
extern crate serde_derive;
use super::*;
use glm::*;
use rand::*;
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;
const HISTORY_DEPTH: usize = 4;
#[derive(Debug, Clone, Default)]
pub struct History {
    pub period: f32, //非ゼロかつ正の値を保証すること
    //過去{HISTORY_DEPTH}分のデータを持っていることが保証される
    rights: BTreeMap<RobotID, [Robot; HISTORY_DEPTH]>,
    lefts: BTreeMap<RobotID, [Robot; HISTORY_DEPTH]>,
    ball: Option<[Ball; HISTORY_DEPTH]>,
}

impl History {
    #[allow(dead_code)]
    pub fn new(period: f32, scenes: &[&Scene; HISTORY_DEPTH]) -> History {
        if period <= 0.0 {
            panic!();
        }

        //過去{HISTORY_DEPTH}分のデーターを持っていたら、抽出する
        let first: &Scene = scenes[0];
        let rights: BTreeMap<_, _> = first
            .rights
            .keys()
            .filter_map(|key| {
                let mut robots = [Robot::default(); HISTORY_DEPTH];
                for i in 0..HISTORY_DEPTH {
                    if let Some(robot) = scenes[i].rights.get(key) {
                        robots[i] = *robot;
                    } else {
                        return None;
                    }
                }
                Some((*key, robots))
            })
            .collect();
        let lefts: BTreeMap<_, _> = first
            .lefts
            .keys()
            .filter_map(|key| {
                let mut robots = [Robot::default(); HISTORY_DEPTH];
                for i in 0..HISTORY_DEPTH {
                    if let Some(robot) = scenes[i].lefts.get(key) {
                        robots[i] = *robot;
                    } else {
                        return None;
                    }
                }
                Some((*key, robots))
            })
            .collect();
        let ball: Option<_> = (|| {
            let mut history = [Ball::default(); HISTORY_DEPTH];
            for i in 0..HISTORY_DEPTH {
                if let Some(ball) = scenes[i].ball {
                    history[i] = ball;
                } else {
                    return None;
                }
            }
            Some(history)
        })();
        History {
            period: period,
            rights: rights,
            lefts: lefts,
            ball: ball,
        }
    }

    pub fn scene_nth(&self, n: usize) -> Option<Scene> {
        if n < 0 || HISTORY_DEPTH <= n {
            return None;
        }

        let rights_iter = self.rights.iter().map(|(key, history)| (*key, history[n]));
        let lefts_iter = self.lefts.iter().map(|(key, history)| (*key, history[n]));
        let ball: Option<Ball> = if let Some(history) = self.ball {
            Some(history[n])
        } else {
            None
        };
        Some(Scene::from_iter(rights_iter, lefts_iter, ball))
    }

    #[inline(always)]
    pub fn scene_now(&self, n: usize) -> Scene {
        self.scene_nth(0).unwrap()
    }

    pub fn simulate<
        RS: FnMut(&[Robot; HISTORY_DEPTH]) -> Robot,
        BS: FnMut(&[Ball; HISTORY_DEPTH]) -> Ball,
    >(
        &self,
        robot_simulate: RS,
        ball_simulate: BS,
    ) -> History {
        let rights: BTreeMap<_, _> = self
            .rights
            .iter()
            .map(|(key, robots)| {
                let new_robot = robot_simulate(robots);
                (key, [new_robot, robots[0], robots[1], robots[2]])
            })
            .collect();
        let lefts: BTreeMap<_, _> = self
            .lefts
            .iter()
            .map(|(key, robots)| {
                let new_robot = robot_simulate(robots);
                (key, [new_robot, robots[0], robots[1], robots[2]])
            })
            .collect();
        let ball = if let Some(balls) = self.ball {
            let new_ball = ball_simulate(&balls);
            Some([new_ball, balls[0], balls[1], balls[2]])
        } else {
            None
        };
        History {
            rights: rights,
            lefts: lefts,
            ball: ball,
        }
    }

    /*
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
    pub fn simulate(&self) -> Scene {
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
     */
}

#[allow(dead_code)]
pub fn tree_plan<G: Fn(&History) -> Scene, SE: Fn(&Scene) -> f32, P: Fn(Scene) -> Option<Scene>>(
    history: &History,
    generator: &G,
    static_evaluation: &SE,
    prune: &P,
    depth: u32,
) -> (f32, Vec<Rc<Scene>>) {
    fn inner<G: Fn(&History) -> Scene, SE: Fn(&Scene) -> f32, P: Fn(Scene) -> Option<Scene>>(
        history: &History,
        generator: &G,
        static_evaluation: &SE,
        prune: &P,
        depth: u32,
    ) -> (f32, Vec<Rc<Scene>>) {
        let branches: Vec<_> = (0..2)
            .flat_map(|_| prune(generator(history)))
            .map(|scene: Scene| {
                let now_score = static_evaluation(&scene);
                let scene = Rc::new(scene);
                if depth == 0 {
                    return (now_score, vec![scene]);
                }
                let fiture = history.push(scene.clone());
                let (next_score, mut scenes) =
                    inner(&fiture, generator, static_evaluation, prune, depth - 1);

                let score = (now_score + next_score) / 2.0;
                scenes.push(scene);
                (score, scenes)
            })
            .collect();
        //find best snene
        let sum: f32 = branches.iter().map(|(score, _)| score).sum();
        let score = sum / (1 << depth) as f32;

        if branches.len() == 0 {
            return (0.0, vec![]);
        }
        let (_, best_branch) = branches
            .into_iter()
            .max_by(|(sa, _), (sb, _)| {
                use std::cmp::Ordering;
                if sa > sb {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            })
            .unwrap();
        (score, best_branch) //strub
    }
    inner(history, generator, static_evaluation, prune, depth)
}
