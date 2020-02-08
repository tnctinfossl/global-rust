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

    let mut graph = ColorGraph::new();
    for point in mine.iter() {
        graph.add_node((*point, color_graph::Color::Blue));
    }
    println!("{}", graph.dump());
    graph.show();
}
