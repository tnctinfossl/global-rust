use super::tree::*;
use std::rc::Rc;

use gnuplot::*;

trait Plotable {
    fn plot<'a>(&self, canvas: &'a mut gnuplot::Figure);
}

impl Plotable for Scene {
    fn plot<'a>(&self, figure: &'a mut Figure) {
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
        axes2d.points(
            blue_xs,
            blue_ys,
            &[PlotOption::Color("blue"), PlotOption::PointSize(5.0)],
        );
        let yellow_xs = yellow_points.iter().map(|p| p.x);
        let yellow_ys = yellow_points.iter().map(|p| p.y);
        axes2d.points(
            yellow_xs,
            yellow_ys,
            &[PlotOption::Color("orange"), PlotOption::PointSize(5.0)],
        );
        let ball_xs = self.balls.values().map(|b| b.position.x);
        let ball_ys = self.balls.values().map(|b| b.position.y);
        axes2d.points(
            ball_xs,
            ball_ys,
            &[PlotOption::Color("red"), PlotOption::PointSize(5.0)],
        );
    }
}

/*#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn plot_scene() {
        let sn = SceneNoise::default();
        let mut figure = gnuplot::Figure::new();
        let scene = Field::default().ramdon_scene(&mut rand::thread_rng(), 10, 10, 1);
        scene.plot(&mut figure);

        std::fs::create_dir_all("img").unwrap();
        figure.save_to_png("img/test_plot.png", 1000, 1000).unwrap();

        scene.noise(&mut rand::thread_rng(),&sn);
        scene.plot(&mut figure);
        
        let mut figure = gnuplot::Figure::new();
        std::fs::create_dir_all("img").unwrap();
        figure.save_to_png("img/test_plot1.png", 1000, 1000).unwrap();
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tree() {
        let sn = &SceneNoise::default();
        let mut figure = gnuplot::Figure::new();
        let field = &Field::default();
        let scene = Rc::new(field.ramdon_scene(&mut rand::thread_rng(), 10, 10, 1));
        let scene1 = Rc::new(
            Rc::try_unwrap(scene.clone())
                .ok()
                .unwrap()
                .noise(&mut rand::thread_rng(), &sn),
        );
        let scene2 = Rc::new(
            Rc::try_unwrap(scene1.clone())
                .ok()
                .unwrap()
                .noise(&mut rand::thread_rng(), &sn),
        );
        let scene3 = Rc::new(
            Rc::try_unwrap(scene2.clone())
                .ok()
                .unwrap()
                .noise(&mut rand::thread_rng(), &sn),
        );
        /*let  scenes:[Rc<Scene>;4] = [Rc::default();4];

        for i in 0..3{
            let scene = Rc::new(Field::default().ramdon_scene(&mut rand::thread_rng(), 10, 10, 1));
            scenes[i] = scene;
        } */
        let scenes = [scene, scene1, scene2, scene3];
        let history = History::new(1.0, scenes);
        //history.simulate(1, &mut rand::thread_rng(), &field);
        let tree = Tree::new(&history);
        tree.new_children(1);
        Rc::try_unwrap(tree.parent.scenes[0].clone()).ok().unwrap().plot(&mut figure);
        figure.save_to_png("img/test_plot.png", 1000, 1000).unwrap();
        Rc::try_unwrap(tree.children[0].scenes[0].clone()).ok().unwrap().plot(&mut figure);
        figure.save_to_png("img/test_plot1.png", 1000, 1000).unwrap();

    }
}