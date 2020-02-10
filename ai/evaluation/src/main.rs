mod common;
mod team;
use common::*;
use glm::*;
use std::time::*;
use team::*;

fn main() {
    /*
    let mut field =BitField::new_points(&[(1,1),(10,10)]);
    println!("{}",field.dump());
    let expand=field.expand4();
    println!("{}",expand.dump());
    let expand=expand.expand4();
    println!("{}",expand.dump());
    */
    let mut gen = rand::thread_rng();
    let field = Rectangle::new(vec2(-64.0, 50.0), vec2(128.0, -100.0));
    let mine: Vec<_> = (0..10).map(|_| field.sample(&mut gen)).collect();
    let yours: Vec<_> = (0..10).map(|_| field.sample(&mut gen)).collect();
    let cell = team::CellDomination2::new(field.clone());

    let mut score = (0.0, 0.0);
    let trial = 1000;
    let mut time = Duration::from_secs(0);
    let max=|a,b|if a>b{a}else {b};

    for _ in 0..trial {
        let begin = Instant::now();
        score = cell.evaluate(&mine, &yours);
        //println!("{:?}",score);
        let end = Instant::now();
        time = max(time, end - begin);
    }
    println!("Old time={:?},score={:?}", time, score);
    println!("mine={:?}",mine);
    println!("yours={:?}",yours);
   
}
