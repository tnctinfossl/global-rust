extern crate model;
//use model::*;
mod evaluation;
mod game;
mod util;
//use evaluation::space_domination;
//use glm::*;
use rand::SeedableRng;

//use std::time::{Duration, Instant};
fn main() {
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
}
