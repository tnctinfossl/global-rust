extern crate model;
use model::*;
use rand;

fn main() {
    let mut world=World::new();
    world.alocate_random(&mut rand::thread_rng(), 8);
}


