pub mod ball;
pub mod field;
pub mod robot;
pub mod scene;
pub mod shape;
pub mod traits;
pub mod history;
pub mod vec2rad;
pub use ball::Ball;
pub use field::Field;
pub use robot::{Robot, RobotID};
pub use scene::{Scene, SceneNoise};
pub use shape::{Circle, Rectangle};
pub use traits::{Overlap, Plotable};
pub use history::{History,tree_plan};
pub use vec2rad::{vec2rad, Vec2Rad};
pub use rand::*;


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::evaluation::*;
    use std::rc::Rc;
    #[test]
    fn plot() {
        let sn = SceneNoise::default();

        let field = Field::default();

        let scene = Field::default().ramdon_scene(&mut rand::thread_rng(), 10, 10, true); //親
        let scene0 = Rc::new(scene.noise(&mut rand::thread_rng(), 10.0, &sn));
        let scene1 = Rc::new(scene.noise(&mut rand::thread_rng(), 10.0, &sn));
        let scene2 = Rc::new(scene.noise(&mut rand::thread_rng(), 10.0, &sn));
        let scene3 = Rc::new(scene.noise(&mut rand::thread_rng(), 10.0, &sn));
        let scenes = [scene0, scene1, scene2, scene3] ;
        
        let history = History::new(1.0, scenes); //親History作成
        //let output = tree_plan(&history,&|&h|h.simulate(),&|&s|scene_passable(&s),&|s|field.prune(s),3);
    }
}