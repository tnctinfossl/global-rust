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

    let mut graph: Graph<Vec2, f32, f32> = Graph::new();
    let node_a = graph.add_node(vec2(0.0, 0.0));
    let node_b = graph.add_node(vec2(1.0, 0.0));
    println!("{:?}", graph[node_a]);
    println!("{:?}", graph);

    //let mut graph: Graph<Vec2, f32, bool> = Graph::new();
}
