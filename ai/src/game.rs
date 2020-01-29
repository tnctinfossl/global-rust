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
pub use history::{History,tree_plan,plot_evaluation};
pub use vec2rad::{vec2rad, Vec2Rad};



#[cfg(test)]
mod tests {
    use super::*;
    use super::super::evaluation::*;
    use std::rc::*;
    pub use rand::*;
    use std::borrow::Borrow;
    #[test]
    fn plot() {
        let sn = SceneNoise::default();
        let mut gen = rand::thread_rng();
        let field = Field::default();
        
        println!("a");
        let scene = Field::default().ramdon_scene(&mut rand::thread_rng(), 10, 10, true); //親
        let scene0 = Rc::new(scene.noise(&mut gen, 10.0, &sn));
        let scene1 = Rc::new(scene0.noise(&mut gen, 10.0, &sn));
        let scene2 = Rc::new(scene1.noise(&mut gen, 10.0, &sn));
        let scene3 = Rc::new(scene2.noise(&mut gen, 10.0, &sn));
        let scenes = [scene0, scene1, scene2, scene3] ;
        
        println!("b");
        let history = History::new(1.0, scenes); //親History作成
        println!("c");
        let snap= move|scene:&Scene|->f32{
            //TODO O(n)かかるからSceneの構造を見直す
            let mut mine = vec![];            
            let mut yours= vec![];
            for (id,robot) in scene.robots.iter(){
                use RobotID::*;
                match id{//TODO あとで矛盾が起きそう
                    Blue(_)=>mine.push(robot.position.to_vec2()),
                    Yellow(_)=>yours.push(robot.position.to_vec2()),
                }
            }
            let (my_score,your_score)=space_domination(&mine[..],&yours[..],&model::Field::new_large());//TODO fieldを統一する
            my_score-your_score
        };

        let (score,best_scenes) = tree_plan(&history,&|h|h.simulate(),&snap,&|s|field.prune(s),3);
        
        
        for (number,s) in best_scenes.iter().enumerate() {
            let mut figure = gnuplot::Figure::new();
            let scene:&Scene=s.borrow();
            scene.plot(&mut figure.axes2d());
            let filename= format!("img/tree_plot{0:02}.png",number);
            figure.save_to_png(&filename,1000, 1000).unwrap();

            
        }

    }
}