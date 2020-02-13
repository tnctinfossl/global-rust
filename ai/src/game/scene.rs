use super::*;
use glm::*;
use gnuplot::*;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use serde_derive::*;
use std::collections::HashMap;
pub static MOVEABLE_DISTANCE: f32 = 4000.0; //1秒間に移動可能な距離[mm]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SceneNoise {
    standard_deviation: f32,     //標準偏差[m]
    standard_deviation_rad: f32, //標準偏差[rad]
}

impl Default for SceneNoise {
    fn default() -> SceneNoise {
        SceneNoise {
            standard_deviation: MOVEABLE_DISTANCE.sqrt(), //1秒間に移動可能な距離[mm]のルート
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
    pub fn noise<R: Rng + ?Sized>(&self, random: &mut R, period: f32, sn: &SceneNoise) -> Scene {
        let robots: HashMap<RobotID, Robot> = self
            .robots
            .iter()
            .map(|(id, robot): (&RobotID, &Robot)| {
                let noised = robot.position + sn.gen_vec2rad(random) * period; //経過時間[s]をかける
                (*id, Robot::new(noised, robot.diametor))
            })
            .collect();
        if let Some(ball) = self.ball {
            let ball = Ball::new(ball.position + sn.gen_vec2(random) * period);
            Scene::new(robots, Some(ball))
        } else {
            Scene::new(robots, None)
        }
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

impl Plotable<gnuplot::Axes2D> for Scene {
    fn plot<'a>(&self, axes2d: &'a mut Axes2D) {
        //let axes2d: &mut Axes2D = figure.axes2d();
        //blue,yellowに分類する
        let mut blue_points: Vec<_> = Vec::new();
        let mut yellow_points: Vec<_> = Vec::new();
        for (id, robot) in &self.robots {
            match id {
                RobotID::Blue(_) => blue_points.push(robot.position),
                RobotID::Yellow(_) => yellow_points.push(robot.position),
            }
        }
        //iteratorとして分解する
        let blue_xs = blue_points.iter().map(|p| p.x);
        let blue_ys = blue_points.iter().map(|p| p.y);
        axes2d.points(
            blue_xs,
            blue_ys,
            &[
                PlotOption::Color("blue"),
                PlotOption::PointSize(10.0),
                PlotOption::PointSymbol('O'),
            ],
        );
        let yellow_xs = yellow_points.iter().map(|p| p.x);
        let yellow_ys = yellow_points.iter().map(|p| p.y);
        axes2d.points(
            yellow_xs,
            yellow_ys,
            &[
                PlotOption::Color("#000000"),
                PlotOption::PointSize(10.0),
                PlotOption::PointSymbol('o'),
            ],
        ); //見やすいように一時的オレンジにした
           /*let ball_xs = self.ball.iter().map(|b| b.position.x);
           let ball_ys = self.ball.iter().map(|b| b.position.y);
           axes2d.points(
               ball_xs,
               ball_ys,
               &[PlotOption::Color("red"), PlotOption::PointSize(5.0),PlotOption::PointSymbol('O')],
           );*/
        let field_x = [-6000, 6000].iter();
        let field_y = [4500, -4500].iter();
        axes2d.points(
            field_x,
            field_y,
            &[
                PlotOption::Color("#FFFFFF"),
                PlotOption::PointSize(1.0),
                PlotOption::PointSymbol('o'),
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
