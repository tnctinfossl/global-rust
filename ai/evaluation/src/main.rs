mod common;
mod team;
use common::*;
use glm::*;
use std::time::Instant;
use team::*;
fn main() {
    let mut gen = rand::thread_rng();
    let field = Rectangle::new(vec2(-64.0, 50.0), vec2(128.0, -100.0));
    let mine: Vec<_> = (0..10).map(|_| field.sample(&mut gen)).collect();
    let yours: Vec<_> = (0..10).map(|_| field.sample(&mut gen)).collect();
    let cell = team::CellDomination::new(field);

    let begin = Instant::now();
    let score = cell.evaluate(&mine, &yours);
    let end = Instant::now();
    println!("time={:?},score={:?}", end - begin, score);
}
