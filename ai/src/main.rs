extern crate model;
//use model::*;
mod evaluation;
mod game;
mod util;
//use evaluation::space_domination;
use glm::*;
use std::rc::*;
use game::*;
use evaluation::*;
use std::borrow::Borrow;
//use rand::SeedableRng;

use std::time::{Duration, Instant};

fn main() {
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

  let sim = move |h: &History| {
      let mut gen_sim = rand::thread_rng();
      h.simulate().noise(&mut gen_sim, 10.0, &sn)
  };
  let start = Instant::now();

  let (_score, best_scenes) = tree_plan(&history, &sim, &snap, &|s| field.prune(s), 3);

  let end = start.elapsed();

  println!("{}.{:03}秒経過しました。", end.as_secs(), end.subsec_nanos() / 1_000_000);
  for (number, s) in best_scenes.iter().enumerate() {
      let mut figure = gnuplot::Figure::new();
      let scene: &Scene = s.borrow();
      scene.plot(&mut figure.axes2d());
      let filename = format!("img/tree_plot{0:02}.png", number);
      figure.save_to_png(&filename, 1200,900).unwrap();
  }

}









  /*  let mut rng = rand_xoshiro::Xoshiro256StarStar::seed_from_u64(123);
  let mut world = model::World::new();
  world.alocate_random(&mut rng, 10);*/
  /*
  let (begin,end)=(world.field.goal(model::Side::Right)+vec2(0.1,0.1),world.field.goal(model::Side::Left));
  let blues=world.blues.robots.iter();
  let yellows=world.yellows.robots.iter();
  let objects :Vec<_>= blues.chain(yellows).map(|p:&Box<model::Robot>|{
      p.position
  }).collect();
  println!("{}",evaluation::passable((begin,end), objects.iter()));
  */

