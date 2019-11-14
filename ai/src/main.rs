extern crate model;
use model::*;
use rand;
mod bitfield;
mod evaluation;
use evaluation::space_domination;
use std::time::{Duration, Instant};
fn main() {
    let mut world=World::new();
    world.alocate_random(&mut rand::thread_rng(), 8);
    let start = Instant::now();
    let  (mut a,mut b)=(0.0,0.0);
    for i in 1..1000{
        let (c,d)=space_domination(&world); 
        a+=c;
        b+=d;
    }
    let end = start.elapsed();
    println!("{}.{:03}秒経過しました。{}{}", end.as_secs(), end.subsec_nanos() / 1_000_000,a,b);

}


