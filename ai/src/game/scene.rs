use super::*;
use glm::*;
use gnuplot::*;
use rand::prelude::ThreadRng;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use serde_derive::*;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    //pub robots: HashMap<RobotID, Robot>,
    pub rights: BTreeMap<RobotID, Robot>,
    pub lefts: BTreeMap<RobotID, Robot>,
    pub ball: Option<Ball>,
}

impl Scene {
    #[allow(dead_code)]
    pub fn new(
        rights: BTreeMap<RobotID, Robot>,
        lefts: BTreeMap<RobotID, Robot>,
        ball: Option<Ball>,
    ) -> Scene {
        Scene {
            rights: rights,
            lefts: lefts,
            ball: ball,
        }
    }
    #[allow(dead_code)]
    pub fn from_iter<
        IR: Iterator<Item = (RobotID, Robot)>,
        IL: Iterator<Item = (RobotID, Robot)>,
    >(
        rights: IR,
        lefts: IL,
        ball: Option<Ball>,
    ) -> Scene {
        Scene {
            rights: rights.collect(),
            lefts: lefts.collect(),
            ball: ball,
        }
    }
}

impl Default for Scene {
    #[allow(dead_code)]
    fn default() -> Scene {
        Scene {
            rights: BTreeMap::new(),
            lefts: BTreeMap::new(),
            ball: None,
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SceneNoise<R> {
    random: R,
    standard_deviation: f32,     //標準偏差[m/s]
    standard_deviation_rad: f32, //標準偏差[rad/s]
}
/*
impl Default for SceneNoise {
    fn default() -> SceneNoise<R: Rng + ?Sized> {
        SceneNoise {
            standard_deviation: 1.2,                      //[m/s]
            standard_deviation_rad: std::f32::consts::PI, //[rad/s]
        }
    }
}*/

impl SceneNoise<ThreadRng> {
    pub fn default() -> SceneNoise<ThreadRng> {
        SceneNoise {
            random: rand::thread_rng(),
            standard_deviation: 1.2,                      //[m/s]
            standard_deviation_rad: std::f32::consts::PI, //[rad/s]
        }
    }
}

impl<R> SceneNoise<R>
where
    R: Rng,
{
    #[allow(dead_code)]
    pub fn new(random: R, standard_deviation: f32, standard_deviation_rad: f32) -> SceneNoise<R> {
        SceneNoise {
            random: random,
            standard_deviation: standard_deviation,
            standard_deviation_rad: standard_deviation_rad,
        }
    }

    pub fn gen_vec2(&mut self, period: f32) -> Vec2 {
        let normal = Normal::new(0.0 as f32, self.standard_deviation as f32 / period).unwrap();
        vec2(
            normal.sample(&mut self.random),
            normal.sample(&mut self.random),
        )
    }

    pub fn gen_vec2rad(&mut self, period: f32) -> Vec2Rad {
        let normal_xy = Normal::new(0.0 as f32, self.standard_deviation as f32 / period).unwrap();
        let normal_theta =
            Normal::new(0.0 as f32, self.standard_deviation_rad as f32 / period).unwrap();
        vec2rad(
            normal_xy.sample(&mut self.random),
            normal_xy.sample(&mut self.random),
            normal_theta.sample(&mut self.random),
        )
    }

    pub fn noise(&mut self, scene: &Scene, period: f32) -> Scene {
        let rights: BTreeMap<RobotID, Robot> = scene
            .rights
            .iter()
            .map(|(key, robot)| {
                (
                    *key,
                    robot.replace(robot.position + self.gen_vec2rad(period)),
                )
            })
            .collect();
        let lefts: BTreeMap<RobotID, Robot> = scene
            .lefts
            .iter()
            .map(|(key, robot)| {
                (
                    *key,
                    robot.replace(robot.position + self.gen_vec2rad(period)),
                )
            })
            .collect();
        let ball = scene
            .ball
            .map(|ball| ball.replace(ball.position + self.gen_vec2(period)));
        Scene::new(rights, lefts, ball)
    }
}

impl Plotable<gnuplot::Axes2D> for Scene {
    fn plot<'a>(&self, axes2d: &'a mut Axes2D) {
        //let axes2d: &mut Axes2D = figure.axes2d();
        //blue,yellowに分類する

        //iteratorとして分解する
        let blue_xs = self.rights.values().map(|r| r.position.x);
        let blue_ys = self.rights.values().map(|r| r.position.y);
        axes2d.points(
            blue_xs,
            blue_ys,
            &[
                PlotOption::Color("blue"),
                PlotOption::PointSize(15.0),
                PlotOption::PointSymbol('O'),
            ],
        );
        let yellow_xs = self.lefts.values().map(|r| r.position.x);
        let yellow_ys = self.lefts.values().map(|r| r.position.y);
        axes2d.points(
            yellow_xs,
            yellow_ys,
            &[
                PlotOption::Color("orange"),
                PlotOption::PointSize(15.0),
                PlotOption::PointSymbol('O'),
            ],
        ); //見やすいように一時的オレンジにした
        let ball_xs = self.ball.iter().map(|b| b.position.x);
        let ball_ys = self.ball.iter().map(|b| b.position.y);
        axes2d.points(
            ball_xs,
            ball_ys,
            &[
                PlotOption::Color("red"),
                PlotOption::PointSize(15.0),
                PlotOption::PointSymbol('O'),
            ],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn plot_scene() {
        let mut sn = SceneNoise::default();
        let mut figure = gnuplot::Figure::new();
        let scene = Field::default().ramdon_scene(&mut rand::thread_rng(), 10, 10, true);
        scene.plot(&mut figure.axes2d());

        std::fs::create_dir_all("img").unwrap();
        figure.save_to_png("img/test_plot.png", 1000, 1000).unwrap();
        let noised_scene = sn.noise(&scene, 16.66);
        //scene.noise(&mut rand::thread_rng(), 16.66, &sn);
        noised_scene.plot(&mut figure.axes2d());
        let mut figure = gnuplot::Figure::new();
        std::fs::create_dir_all("img").unwrap();
        figure
            .save_to_png("img/test_plot1.png", 1000, 1000)
            .unwrap();
    }
}
