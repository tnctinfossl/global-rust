use glm::*;
use std::borrow::*;
use std::cell::*;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::*;
use std::rc::*;
use typed_arena::Arena;
#[derive(Debug, Clone, Copy)]
pub struct NodeId {
    id: usize,
}

impl From<usize> for NodeId {
    fn from(id: usize) -> NodeId {
        NodeId { id: id }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct EdgeId {
    begin: usize,
    end: usize,
}

impl From<(NodeId, NodeId)> for EdgeId {
    fn from((begin, end): (NodeId, NodeId)) -> EdgeId {
        EdgeId {
            begin: begin.id,
            end: end.id,
        }
    }
}

impl From<(usize, usize)> for EdgeId {
    fn from((begin, end): (usize, usize)) -> EdgeId {
        EdgeId {
            begin: begin,
            end: end,
        }
    }
}

impl EdgeId {
    pub fn end(&self) -> NodeId {
        NodeId { id: self.end }
    }
}

#[derive(Debug, Clone)]
pub struct Graph<N, E> {
    nodes: Vec<N>,
    edges: HashMap<EdgeId, E>,
}

impl<N, E> Graph<N, E> {
    #[allow(dead_code)]
    pub fn new() -> Graph<N, E> {
        Graph {
            nodes: vec![],
            edges: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_node(&mut self, node: N) -> NodeId {
        self.nodes.push(node);
        //result
        NodeId::from(self.nodes.len() - 1)
    }

    #[allow(dead_code)]
    pub fn add_edge(&mut self, (begin, end): (NodeId, NodeId), edge: E) -> EdgeId {
        let id = EdgeId::from((begin, end));
        self.edges.insert(id, edge);
        id
    }

    #[allow(dead_code)]
    pub fn get_node(&self, id: NodeId) -> &N {
        &self.nodes[id.id]
    }

    #[allow(dead_code)]
    pub fn get_edge(&self, id: EdgeId) -> &E {
        &self.edges[&id]
    }
    #[allow(dead_code)]
    pub fn begin(&self, begin: NodeId) -> Vec<EdgeId> {
        (0..self.nodes.len())
            .filter_map(|end: usize| {
                let id = EdgeId::from((begin.id, end));
                if let Some(_) = self.edges.get(&id) {
                    Some(id)
                } else {
                    None
                }
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn end(&self, end: NodeId) -> Vec<EdgeId> {
        (0..self.nodes.len())
            .filter_map(|begin: usize| {
                let id = EdgeId::from((begin, end.id));
                if let Some(_) = self.edges.get(&id) {
                    Some(id)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl<N, E> Index<NodeId> for Graph<N, E> {
    type Output = N;
    fn index(&self, id: NodeId) -> &N {
        &self.nodes[id.id]
    }
}

impl<N, E> IndexMut<NodeId> for Graph<N, E> {
    fn index_mut(&mut self, id: NodeId) -> &mut N {
        &mut self.nodes[id.id]
    }
}

impl<N, E> Index<EdgeId> for Graph<N, E> {
    type Output = E;
    fn index(&self, id: EdgeId) -> &E {
        &self.edges[&id]
    }
}

impl<N, E> IndexMut<EdgeId> for Graph<N, E> {
    fn index_mut(&mut self, id: EdgeId) -> &mut E {
        self.edges.get_mut(&id).unwrap()
    }
}
