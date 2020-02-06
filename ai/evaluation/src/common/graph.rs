use glm::*;
use std::borrow::*;
use std::cell::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::*;
use std::rc::*;
use typed_arena::Arena;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Node(usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edge(usize, usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Face(usize, usize, usize);

#[derive(Debug, Clone, Default)]
pub struct Graph<N, E, F> {
    nodes: Vec<N>,
    edeges: HashMap<(usize, usize), E>,
    faces: HashMap<(usize, usize, usize), F>,
}

impl<N, E, F> Graph<N, E, F> {
    #[allow(dead_code)]
    pub fn new() -> Graph<N, E, F> {
        Graph {
            nodes: Vec::new(),
            edeges: HashMap::new(),
            faces: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_node(&mut self, value: N) -> Node {
        let result = Node(self.nodes.len());
        self.nodes.push(value);
        result
    }
    #[allow(dead_code)]
    pub fn add_edge(&mut self, (begin, end): (Node, Node), value: E) -> Edge {
        //self.edeges[&] = value;
        self.edeges.insert((begin.0, end.0), value);
        Edge(begin.0, end.0)
    }
}

impl<N, E, F> Index<Node> for Graph<N, E, F> {
    type Output = N;
    fn index(&self, index: Node) -> &N {
        &self.nodes[index.0]
    }
}

impl<N, E, F> IndexMut<Node> for Graph<N, E, F> {
    fn index_mut(&mut self, index: Node) -> &mut N {
        &mut self.nodes[index.0]
    }
}

impl<N, E, F> Index<Edge> for Graph<N, E, F> {
    type Output = E;
    fn index<'a>(&'a self, index: Edge) -> &'a E {
        &self.edeges[&(index.0, index.1)]
    }
}

impl<N, E, F> IndexMut<Edge> for Graph<N, E, F> {
    fn index_mut(&mut self, index: Edge) -> &mut E {
        self.edeges.get_mut(&(index.0, index.1)).unwrap()
    }
}

impl<N, E, F> Index<Face> for Graph<N, E, F> {
    type Output = F;
    fn index<'a>(&'a self, index: Face) -> &'a F {
        &self.faces[&(index.0, index.1, index.2)]
    }
}

impl<N, E, F> IndexMut<Face> for Graph<N, E, F> {
    fn index_mut(&mut self, index: Face) -> &mut F {
        self.faces.get_mut(&(index.0, index.1, index.2)).unwrap()
    }
}
