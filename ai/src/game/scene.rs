use super::*;
use glm::*;
use gnuplot::*;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use serde_derive::*;
use std::collections::{BTreeMap, HashMap};
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SceneNoise {
    standard_deviation: f32,     //標準偏差[m]
    standard_deviation_rad: f32, //標準偏差[rad]
}

impl Default for SceneNoise {
    fn default() -> SceneNoise {
        SceneNoise {
            standard_deviation: 1200.0, //[mm]
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

    #[allow(dead_code)]
    pub fn noise<R: Rng + ?Sized>(&self, random: &mut R, period: f32, sn: &SceneNoise) -> Scene {
        let rights: BTreeMap<RobotID, Robot> = self
            .rights
            .iter()
            .map(|(id, robot)| {
                let noised = robot.position + sn.gen_vec2rad(random) / period;
                (*id, Robot::new(noised, robot.diametor))
            })
            .collect();
        let lefts: BTreeMap<RobotID, Robot> = self
            .lefts
            .iter()
            .map(|(id, robot)| {
                let noised = robot.position + sn.gen_vec2rad(random) / period;
                (*id, Robot::new(noised, robot.diametor))
            })
            .collect();
        let mut ball = None;
        if let Some(b) = self.ball {
            ball = Some(Ball::new(b.position + sn.gen_vec2(random) / period));
        }

        Scene {
            rights: rights,
            lefts: lefts,
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
        let sn = SceneNoise::default();
        let mut figure = gnuplot::Figure::new();
        let scene = Field::default().ramdon_scene(&mut rand::thread_rng(), 10, 10, true);
        scene.plot(&mut figure.axes2d());

        std::fs::create_dir_all("img").unwrap();
        figure.save_to_png("img/test_plot.png", 1000, 1000).unwrap();

        scene.noise(&mut rand::thread_rng(), 16.66, &sn);
        scene.plot(&mut figure.axes2d());
        let mut figure = gnuplot::Figure::new();
        std::fs::create_dir_all("img").unwrap();
        figure
            .save_to_png("img/test_plot1.png", 1000, 1000)
            .unwrap();
    }
}
