use glm::*;
use std::borrow::*;
use std::cell::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::*;
use std::rc::*;
use typed_arena::Arena;

pub struct Node<'a, N, E> {
    value: N,
    edges: RefCell<Vec<Edge<'a, N, E>>>,
}

impl<'a, N, E> Node<'a, N, E> {
    fn new(value: N) -> Node<'a, N, E> {
        Node {
            value: value,
            edges: RefCell::new(vec![]),
        }
    }
    #[allow(dead_code)]
    pub fn connect(&self, node: &'a Self, value: E) {
        let mut edges = self.edges.borrow_mut();
        let index = edges.len() - 1;
        edges.push(Edge::new(node, value));
    }
    #[allow(dead_code)]
    pub fn edges(&'a self) -> Ref<'a, Vec<Edge<'a, N, E>>> {
        self.edges.borrow()
    }
}

impl<'a, N, E> Deref for Node<'a, N, E> {
    type Target = N;
    fn deref(&self) -> &N {
        &self.value
    }
}

impl<'a, N, E> DerefMut for Node<'a, N, E> {
    fn deref_mut(&mut self) -> &mut N {
        &mut self.value
    }
}

pub struct Edge<'a, N, E> {
    node: &'a Node<'a, N, E>,
    value: E,
}

impl<'a, N, E> Edge<'a, N, E> {
    fn new(node: &'a Node<'a, N, E>, value: E) -> Edge<'a, N, E> {
        Edge {
            node: node,
            value: value,
        }
    }
    #[allow(dead_code)]
    fn node(&self) -> &'a Node<'a, N, E> {
        self.node
    }
}

impl<'a, N, E> Deref for Edge<'a, N, E> {
    type Target = E;
    fn deref(&self) -> &E {
        &self.value
    }
}

impl<'a, N, E> DerefMut for Edge<'a, N, E> {
    fn deref_mut(&mut self) -> &mut E {
        &mut self.value
    }
}

//無方向グラフ
pub struct Graph<'a, N, E> {
    arena: Arena<Node<'a, N, E>>,
}

impl<'a, N, E> Graph<'a, N, E> {
    #[allow(dead_code)]
    pub fn new<'b>() -> Graph<'b, N, E> {
        Graph {
            arena: Arena::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_node<'b>(&self, value: N) -> &mut Node<'a, N, E> {
        self.arena.alloc(Node::new(value))
    }
}
