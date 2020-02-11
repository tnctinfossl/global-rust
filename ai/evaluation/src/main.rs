mod common;
mod team;
use common::*;
use glm::*;
use std::cell::*;
use std::rc::*;
use std::time::Instant;
use team::*;
fn main() {
    use glm;
    use rand::*;
    let mut gen = rand::thread_rng();
    let field = *common::consts::FIELD_A;
    let goals = *common::consts::GOALS_A;
    let mine: Vec<_> = (0..10).map(|_| field.sample(&mut gen)).collect();
    let yours: Vec<_> = (0..10).map(|_| field.sample(&mut gen)).collect();

    let dominater = FieldDomination::new(field, (640, 500));
    dominater.show(&mine, &yours);
}
