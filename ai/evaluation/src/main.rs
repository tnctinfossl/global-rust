mod common;
mod team;
use common::*;
use glm::*;
use std::rc::*;
use std::time::Instant;
use team::*;
fn main() {
    use glm;
    use rand::*;
    let mut gen = rand::thread_rng();
    let field = Rectangle::new(vec2(-64.0, 50.0), vec2(128.0, -100.0));
    let mine: Vec<_> = (0..10).map(|_| field.sample(&mut gen)).collect();

    let mut graph: Graph<i32, char> = Graph::new();

    let na = graph.add_node(0);
    let nb = graph.add_node(1);
    let nc = graph.add_node(2);
    let nd = graph.add_node(3);

    let eab = graph.add_edge((na, nb), 'x');
    let ebc = graph.add_edge((nb, nc), 'y');
    let ebd = graph.add_edge((nb, nd), 'z');

    show(&graph, na);
}

fn show(graph: &Graph<i32, char>, node: NodeId) {
    fn inner(graph: &Graph<i32, char>, node_id: NodeId) {
        println!("{}", graph[node_id]);
        for (edge_id, node_id) in graph.front_id(node_id).iter() {
            println!("{}", graph[*edge_id]);
            inner(graph, *node_id);
        }
    }
    inner(graph, node);
}
