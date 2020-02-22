extern crate model;
extern crate serde;
extern crate serde_derive;
use super::*;
use glm::*;
use rand::Rng;
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
    pub fn scene_now(&self) -> Scene {
        self.scene_nth(0).unwrap()
    }
    #[allow(dead_code)]
    pub fn generate<
        RS: FnMut(&[Robot; HISTORY_DEPTH]) -> Option<Robot>,
        BS: FnMut(&[Ball; HISTORY_DEPTH]) -> Option<Ball>,
    >(
        &self,
        mut robot_simulate: RS,
        mut ball_simulate: BS,
    ) -> History {
        let rights: BTreeMap<_, _> = self
            .rights
            .iter()
            .flat_map(|(key, robots)| {
                robot_simulate(robots)
                    .map(|new_robot| (*key, [new_robot, robots[0], robots[1], robots[2]]))
            })
            .collect();
        let lefts: BTreeMap<_, _> = self
            .lefts
            .iter()
            .flat_map(|(key, robots)| {
                robot_simulate(robots)
                    .map(|new_robot| (*key, [new_robot, robots[0], robots[1], robots[2]]))
            })
            .collect();

        let ball = if let Some(ball_history) = self.ball {
            ball_simulate(&ball_history)
                .map(|new_ball| [new_ball, ball_history[0], ball_history[1], ball_history[2]])
        } else {
            None
        };
        History {
            period: self.period,
            rights: rights,
            lefts: lefts,
            ball: ball,
        }
    }
    //物理シミュレーション
    pub fn simulate_physics(&self) -> History {
        let t1 = self.period;
        let t2 = self.period.powi(2);
        let t3 = self.period.powi(3);
        let rs = |robots: &[Robot; HISTORY_DEPTH]| -> Option<Robot> {
            //物理シミュレーション
            let p0 = robots[0].position;
            let v = (robots[0].position - robots[1].position) / self.period;
            let a = (robots[0].position - 2.0 * robots[1].position + robots[2].position)
                / (self.period * 2.0);
            let j = (robots[0].position - 3.0 * robots[1].position + 3.0 * robots[2].position
                - robots[3].position)
                / (self.period * 3.0);
            let p = j / 6.0 * t3 + a / 2.0 * t2 + v * t1 + p0;
            //ノイズ付与
            Some(robots[0].replace(p))
        };
        let bs = |balls: &[Ball; HISTORY_DEPTH]| -> Option<Ball> {
            //物理シミュレーション
            let p0 = balls[0].position;
            let v = (balls[0].position - balls[1].position) / self.period;
            let a = (balls[0].position - balls[1].position * 2.0 + balls[2].position)
                / (self.period * 2.0);
            let j = (balls[0].position - balls[1].position * 3.0 + balls[2].position * 3.0
                - balls[3].position)
                / (self.period * 3.0);
            let p = j / 6.0 * t3 + a / 2.0 * t2 + v * t1 + p0;
            //fin
            Some(balls[0].replace(p))
        };
        self.generate(rs, bs)
    }

    //現在の位置にノイズを与える。
    pub fn noise<R: Rng>(&self, sn: &mut SceneNoise<R>) -> History {
        let mut history = self.clone();
        for right in history.rights.values_mut() {
            right[0] = right[0].replace(right[0].position + sn.gen_vec2rad(self.period));
        }

        for left in history.lefts.values_mut() {
            left[0] = left[0].replace(left[0].position + sn.gen_vec2rad(self.period));
        }

        if let Some(ball) = history.ball {
            history.ball.unwrap()[0] = ball[0].replace(ball[0].position + sn.gen_vec2(self.period));
        }
        history
    }
}

#[allow(dead_code)]
pub fn tree_plan<
    G: Fn(&History) -> Scene,
    SE: Fn(&Scene) -> f32,
    P: Fn(&Scene) -> Option<Scene>,
>(
    history: &History,
    generator: &G,
    static_evaluation: &SE,
    prune: &P,
    depth: u32,
) -> (f32, Vec<Scene>) {
    fn inner<G: Fn(&History) -> Scene, SE: Fn(&Scene) -> f32, P: Fn(&Scene) -> Option<Scene>>(
        history: &History,
        generator: &G,
        static_evaluation: &SE,
        prune: &P,
        depth: u32,
    ) -> (f32, Vec<Scene>) {
        const n: usize = 2;
        let mut branches = Vec::with_capacity(n);
        let mut scene_noise = SceneNoise::default(); //あとで解決する
        let physics = history.simulate_physics();
        for _ in 0..n {
            let noised = physics.noise(&mut scene_noise);
            let now = noised.scene_now();
            if prune(&now).is_none() {
                continue;
            }
            let (fiture_score, mut scenes) =
                inner(&noised, generator, static_evaluation, prune, depth - 1);

            let score = (fiture_score + static_evaluation(&now)) / 2.0;
            scenes.push(now);
            branches.push((score, scenes));
        }

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
