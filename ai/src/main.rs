extern crate model;
use model::*;
use rand;
mod bitfield;
mod evaluation;
mod util;
mod geometry;
use evaluation::space_domination;
use glm::*;
use geometry::*;
use std::time::{Duration, Instant};
fn main() {
    let d=distance_segment_point((vec2(0.0,0.0),vec2(2.0,0.0)), vec2(1.0,3.0));
    //assert_eq!(d,1.0);
    println!("{}",d);
}


