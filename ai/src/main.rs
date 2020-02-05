extern crate evaluation;
extern crate model;
//use model::*;

mod game;
mod util;
use evaluation::*;
use game::*;
use glm::*;
use std::borrow::Borrow;
use std::rc::*;
//use rand::SeedableRng;
use crate::common::Rectangle;
use std::collections::HashMap;
use std::time::{Duration, Instant};

fn main() {
    //手動シチュエーション
    let mut robots: HashMap<RobotID, Robot> = HashMap::new();
    robots.insert(
        RobotID::Blue(0),
        Robot::new(vec2rad(-5500.0, 0.0, 0.0), 0.1),
    );
    robots.insert(
        RobotID::Blue(1),
        Robot::new(vec2rad(-4000.0, 3500.0, 0.0), 0.1),
    );
    robots.insert(
        RobotID::Blue(2),
        Robot::new(vec2rad(-4000.0, -3500.0, 0.0), 0.1),
    );
    robots.insert(
        RobotID::Blue(3),
        Robot::new(vec2rad(-3000.0, 2500.0, 0.0), 0.1),
    );
    robots.insert(
        RobotID::Blue(4),
        Robot::new(vec2rad(-3000.0, -2500.0, 0.0), 0.1),
    );
    robots.insert(
        RobotID::Blue(5),
        Robot::new(vec2rad(-3500.0, 0.0, 0.0), 0.1),
    );
    /* robots.insert(
        RobotID::Blue(6),
        Robot::new(vec2rad(4400.0, -1800.0, 0.0), 0.1),
    );
    robots.insert(
        RobotID::Blue(7),
        Robot::new(vec2rad(4500.0, -3100.0, 0.0), 0.1),
    );
    robots.insert(
        RobotID::Blue(8),
        Robot::new(vec2rad(4600.0, -4000.0, 0.0), 0.1),
    );
    robots.insert(
        RobotID::Blue(9),
        Robot::new(vec2rad(4800.0, -5800.0, 0.0), 0.1),
    );*/
    robots.insert(
        RobotID::Yellow(0),
        Robot::new(vec2rad(5500.0, 0.0, 0.0), 0.1),
    );
    robots.insert(
        RobotID::Yellow(1),
        Robot::new(vec2rad(0.0, 3000.0, 0.0), 0.1),
    );
    robots.insert(
        RobotID::Yellow(2),
        Robot::new(vec2rad(0.0, -3000.0, 0.0), 0.1),
    );
    robots.insert(
        RobotID::Yellow(3),
        Robot::new(vec2rad(-2000.0, 3500.0, 0.0), 0.1),
    );
    robots.insert(
        RobotID::Yellow(4),
        Robot::new(vec2rad(-2000.0, -3500.0, 0.0), 0.1),
    );
    robots.insert(
        RobotID::Yellow(5),
        Robot::new(vec2rad(-1500.0, 0.0, 0.0), 0.1),
    );
    /* robots.insert(
        RobotID::Yellow(6),
        Robot::new(vec2rad(4800.0, -500.0, 0.0), 0.1),
    );
    robots.insert(
        RobotID::Yellow(7),
        Robot::new(vec2rad(4800.0, -800.0, 0.0), 0.1),
    );
    robots.insert(
        RobotID::Yellow(8),
        Robot::new(vec2rad(4800.0, -1000.0, 0.0), 0.1),
    );
    robots.insert(
        RobotID::Yellow(9),
        Robot::new(vec2rad(4800.0, -1300.0, 0.0), 0.1),
    );*/

    let ball = Ball::new(vec2(0.0, 0.0));

    let scene = Scene::new(robots, Some(ball));

    let bitfield = Rectangle::new(vec2(-6000.0, 4500.0), vec2(12000.0, -9000.0));
    let cell = team::CellDomination::new(bitfield);
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
    let (my_score, your_score) = cell.evaluate(&mine[..], &yours[..]);
    let before = my_score;
    println!("before:{}", my_score);
    println!("before{}", your_score);

    let mut figure = gnuplot::Figure::new();
    scene.plot(&mut figure.axes2d());

    std::fs::create_dir_all("img").unwrap();
    figure
        .save_to_png("img/test_plot000.png", 1200, 900)
        .unwrap();

    /*use crate::common::Rectangle;
    let bitfield = Rectangle::new(vec2(-6000.0, 4500.0), vec2(12000.0, -9000.0));
    let cell = team::CellDomination::new(bitfield);
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
    let (my_score, your_score) = cell.evaluate(&mine[..], &yours[..]);
    println!("{}", my_score);
    println!("{}", your_score);*/

    //実行時間計測
    
    let sn = SceneNoise::default();
    let mut gen = rand::thread_rng();
    let field = Field::default();
    //let scene = Field::default().ramdon_scene(&mut rand::thread_rng(), 10, 10, true); //親
    let scene0 = Rc::new(scene);
    let scene1 = Rc::new(scene0.noise(&mut gen, 10.0, &sn));
    let scene2 = Rc::new(scene1.noise(&mut gen, 10.0, &sn));
    let scene3 = Rc::new(scene2.noise(&mut gen, 10.0, &sn));
    let scenes = [scene0, scene1, scene2, scene3];
    let history = History::new(1.0, scenes); //親History作
    let bitfield = Rectangle::new(vec2(-6000.0, 4500.0), vec2(12000.0, -9000.0));
    let cell = team::CellDomination::new(bitfield);
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
        let (my_score, your_score) = cell.evaluate(&mine[..], &yours[..]);
        //println!("myscoreは{},yourscoreは{}", my_score, your_score);
        my_score - your_score
    };

    let sim = move |h: &History| {
        let mut gen_sim = rand::thread_rng();
        h.simulate().noise(&mut gen_sim, 1.0, &sn)
    };
   
    let start = Instant::now();
    let (_score, best_scenes) = tree_plan(&history, &sim, &snap, &|s| field.prune(s), 4);

    let end = start.elapsed();

    println!(
        "{}.{:03}秒経過しました。",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
    for (number, s) in best_scenes.iter().enumerate() {
        let mut figure = gnuplot::Figure::new();
        let scene: &Scene = s.borrow();
        scene.plot(&mut figure.axes2d());
        let filename = format!("img/tree_plot{0:02}.png", number);
        figure.save_to_png(&filename, 1200, 900).unwrap();

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
        let bitfield = Rectangle::new(vec2(-6000.0, 4500.0), vec2(12000.0, -9000.0));
        let cell = team::CellDomination::new(bitfield);
        let (my_score, your_score) = cell.evaluate(&mine[..], &yours[..]);
        let difference = my_score - before;
        println!("after:{}", my_score);
        println!("after:{}", your_score);
        println!("difference:{}", difference);
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
