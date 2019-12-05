extern crate model;
use model::*;
mod bitfield;
mod evaluation;
mod util;
mod geometry;
mod game;
use evaluation::space_domination;
use glm::*;
use geometry::*;
use std::time::{Duration, Instant};
use rand::{Rng, SeedableRng};
fn main() {
    let mut rng = rand_xoshiro::Xoshiro256StarStar::seed_from_u64(123);
    let mut world=model::World::new();
    world.alocate_random( &mut rng, 10);

    println!("{:?}",evaluation::evaluate_shoot(&world.field, &world.blues, &world.yellows));
}


