pub mod ball;
pub mod field;
pub mod history;
pub mod robot;
pub mod scene;
pub mod shape;
pub mod traits;
pub mod vec2rad;
pub use ball::Ball;
pub use field::Field;
pub use history::{tree_plan, History};
pub use robot::{Robot, RobotID};
pub use scene::{Scene, SceneNoise};
pub use shape::{Circle, Rectangle};
pub use traits::{Overlap, Plotable};
pub use vec2rad::{vec2rad, Vec2Rad};

#[cfg(test)]
mod tests {
    use super::super::evaluation::*;
    use super::*;
    use glm::*;
    pub use rand::*;
    use std::borrow::Borrow;
    use std::collections::HashMap;
    use std::rc::*;

    #[test]

    fn plot() {
        let sn = SceneNoise::default();
        let mut gen = rand::thread_rng();
        let field = Field::default();
        let scene = Field::default().ramdon_scene(&mut rand::thread_rng(), 10, 10, true); //親
        let scene0 = Rc::new(scene.noise(&mut gen, 10.0, &sn));
        let scene1 = Rc::new(scene0.noise(&mut gen, 10.0, &sn));
        let scene2 = Rc::new(scene1.noise(&mut gen, 10.0, &sn));
        let scene3 = Rc::new(scene2.noise(&mut gen, 10.0, &sn));
        let scenes = [scene0, scene1, scene2, scene3];
        let history = History::new(1.0, scenes); //親History作

        let snap = move |scene: &Scene| -> f32 {
            //TODO O(n)かかるからSceneの構造を見直す
            let mut mine = vec![];
            let mut yours = vec![];
            for (id, robot) in scene.robots.iter() {
                use RobotID::*;
                match id {
                    //TODO あとで矛盾が起きそう
                    Blue(_) => mine.push(robot.position.to_vec2()),
                    Yellow(_) => yours.push(robot.position.to_vec2()),
                }
            }
            let (my_score, your_score) =
                space_domination(&mine[..], &yours[..], &model::Field::new_large()); //TODO fieldを統一する
            my_score - your_score
        };

        /* let pass_evaluation = move|scene: &Scene| ->f32{
            let mut sum:f32 = 0.0;
            for i in 0..5 {
                sum +=
            }
        }*/

        let sim = move |h: &History| {
            let mut gen_sim = rand::thread_rng();
            h.simulate().noise(&mut gen_sim, 10.0, &sn)
        };

        let (_score, best_scenes) = tree_plan(&history, &sim, &snap, &|s| field.prune(s), 2);

        for (number, s) in best_scenes.iter().enumerate() {
            let mut figure = gnuplot::Figure::new();
            let scene: &Scene = s.borrow();
            scene.plot(&mut figure.axes2d());
            let filename = format!("img/tree_plot{0:02}.png", number);
            figure.save_to_png(&filename, 400, 300).unwrap();
        }
    }
}

/*#[cfg(test)]

mod bench{
    use super::super::evaluation::*;
    use super::*;
    pub use rand::*;
    use std::borrow::Borrow;
    use std::rc::*;
    #![feature(test)]
    extern crate  test;
    use test::Bencher;
    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        let sn = SceneNoise::default();
        let mut gen = rand::thread_rng();
        let field = Field::default();
        let scene = Field::default().ramdon_scene(&mut rand::thread_rng(), 10, 10, true); //親
        let scene0 = Rc::new(scene.noise(&mut gen, 10.0, &sn));
        let scene1 = Rc::new(scene0.noise(&mut gen, 10.0, &sn));
        let scene2 = Rc::new(scene1.noise(&mut gen, 10.0, &sn));
        let scene3 = Rc::new(scene2.noise(&mut gen, 10.0, &sn));
        let scenes = [scene0, scene1, scene2, scene3];
        let history = History::new(1.0, scenes); //親History作

        let snap = move |scene: &Scene| -> f32 {
            //TODO O(n)かかるからSceneの構造を見直す
            let mut mine = vec![];
            let= Rc::new(scene.noise(&mut gen, 10.0, &sn));
            let scene1 = Rc::new(scene0.noise(&mut gen, 10.0, &sn));
            let scene2 = Rc::new(scene1.noise(&mut gen, 10.0, &sn));
            let scene3 = Rc::new(scene2.noise(&mut gen, 10.0, &sn));
            let scenes = [scene0, scene1, scene2, scene3];
            let (my_score, your_score) =
                space_domination(&mine[..], &yours[..], &model::Field::new_large()); //TODO fieldを統一する
            my_score - your_score
        };

        let sim = move |h: &History| {
            let mut gen_sim = rand::thread_rng();
            h.simulate().noise(&mut gen_sim, 10.0, &sn)
        };
        b.iter(|| {
            tree_plan(&history, &sim, &snap, &|s| field.prune(s), 3);
        };
    }
}*/
