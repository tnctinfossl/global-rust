extern crate gnuplot;
use super::tree::*;

use gnuplot::*;

trait Plotable {
    fn plot<'a>(&self, canvas: &'a mut gnuplot::Figure) -> &'a mut Figure;
}

impl Plotable for Scene {
    fn plot<'a>(&self, figure: &'a mut Figure) -> &'a mut Figure {
        let axes2d: &mut Axes2D = figure.axes2d();
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
        axes2d.points(blue_xs, blue_ys, &[PlotOption::Color("blue")]);
        let yellow_xs = yellow_points.iter().map(|p| p.x);
        let yellow_ys = yellow_points.iter().map(|p| p.y);
        axes2d.points(yellow_xs, yellow_ys, &[PlotOption::Color("yellow")]);
        figure
    }
}

#[test]
fn test_plot() {
    let mut figure = gnuplot::Figure::new();
    let scene = Field::default().ramdon_scene(&mut rand::thread_rng(), 10, 10, 1);
    scene.plot(&mut figure);
    figure.save_to_png("test_plot.png", 1000, 1000);
}
