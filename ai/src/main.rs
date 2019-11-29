extern crate model;
use model::*;
mod bitfield;
mod evaluation;
mod util;
mod geometry;
use evaluation::space_domination;
use glm::*;
use geometry::*;
use std::time::{Duration, Instant};
use rand::{Rng, SeedableRng};
fn main() {
    //let mut rng = rand_xoshiro::Xoshiro256StarStar::seed_from_u64(123);
    //let mut world=model::World::new();
    //world.alocate_random( &mut rng, 10);
    //println!("{:?}",evaluation::evaluate_shoot(&world.field, &world.blues, &world.yellows));
    let begin=vec2(1.0,1.0);
    let end=vec2(3.0,3.0);
    let objects=[vec2(1.0,3.0)];
    println!("{:?}",evaluation::passable((begin,end), objects.iter()));

}


