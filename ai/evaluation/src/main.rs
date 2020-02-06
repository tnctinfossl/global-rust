mod common;
mod team;
use common::*;
use glm::*;
use std::time::Instant;
use team::*;
fn main() {
    use glm;
    use rand::*;
    let mut gen = rand::thread_rng();
    let field = Rectangle::new(vec2(-64.0, 50.0), vec2(128.0, -100.0));
    let mine: Vec<_> = (0..10).map(|_| field.sample(&mut gen)).collect();

    let mut graph: Graph<i32, ()> = Graph::new();
    let mut a = graph.add_node(0);
    let mut b = graph.add_node(1);
    let c = graph.add_node(2);
    let d = graph.add_node(3);

    a.connect(b, ());
    b.connect(c, ());
    b.connect(d, ());
    //let mut graph: Graph<Vec2, f32, bool> = Graph::new();
}
