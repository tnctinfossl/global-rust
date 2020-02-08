use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Write;
use std::ops::*;
#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
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
    #[allow(dead_code)]
    pub fn begin(&self) -> NodeId {
        NodeId { id: self.begin }
    }
    #[allow(dead_code)]
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
    pub fn list_nodes(&self) -> Vec<(NodeId, &N)> {
        self.nodes
            .iter()
            .enumerate()
            .map(|(id, value)| (NodeId::from(id), value))
            .collect()
    }
    #[allow(dead_code)]
    pub fn get_nodes(&self) -> &[N] {
        &self.nodes
    }
    #[allow(dead_code)]
    pub fn get_edges(&self) -> &HashMap<EdgeId, E> {
        &self.edges
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
    pub fn front(&self, begin_id: NodeId) -> Vec<(&E, NodeId)> {
        (0..self.nodes.len())
            .filter_map(|end: usize| {
                let end_id = NodeId::from(end);
                let edge_id = EdgeId::from((begin_id, end_id));
                if let Some(edge) = self.edges.get(&edge_id) {
                    Some((edge, end_id))
                } else {
                    None
                }
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn front_id(&self, begin_id: NodeId) -> Vec<(EdgeId, NodeId)> {
        (0..self.nodes.len())
            .filter_map(|end_id: usize| {
                let end_id = NodeId::from(end_id);
                let edge_id = EdgeId::from((begin_id, end_id));
                if let Some(_) = self.edges.get(&edge_id) {
                    Some((edge_id, end_id))
                } else {
                    None
                }
            })
            .collect()
    }
    #[allow(dead_code)]
    pub fn back(&self, end_id: NodeId) -> Vec<(&E, NodeId)> {
        (0..self.nodes.len())
            .filter_map(|begin_id: usize| {
                let begin_id = NodeId::from(begin_id);
                let edge_id = EdgeId::from((begin_id, end_id));
                if let Some(edge) = self.edges.get(&edge_id) {
                    Some((edge, begin_id))
                } else {
                    None
                }
            })
            .collect()
    }
    #[allow(dead_code)]
    pub fn back_id(&self, end_id: NodeId) -> Vec<(EdgeId, NodeId)> {
        (0..self.nodes.len())
            .filter_map(|begin_id: usize| {
                let begin_id = NodeId::from(begin_id);
                let edge_id = EdgeId::from((begin_id, end_id));
                if let Some(_) = self.edges.get(&edge_id) {
                    Some((edge_id, begin_id))
                } else {
                    None
                }
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn joint(&self, id: NodeId) -> Vec<(&E, NodeId)> {
        let id = id.id;
        let xs = (0..self.nodes.len()).filter_map(|end_id| {
            let edge_id = EdgeId::from((id, end_id));
            if let Some(edge) = self.edges.get(&edge_id) {
                Some((edge, NodeId::from(end_id)))
            } else {
                None
            }
        });
        let ys = (0..self.nodes.len()).filter_map(|begin_id| {
            if begin_id == id {
                None
            } else {
                let edge_id = EdgeId::from((begin_id, id));
                if let Some(edge) = self.edges.get(&edge_id) {
                    Some((edge, NodeId::from(begin_id)))
                } else {
                    None
                }
            }
        });
        xs.chain(ys).collect()
    }

    #[allow(dead_code)]
    pub fn joint_id(&self, id: NodeId) -> Vec<(EdgeId, NodeId)> {
        let id = id.id;
        let xs = (0..self.nodes.len()).filter_map(|end_id| {
            let edge_id = EdgeId::from((id, end_id));
            if let Some(_) = self.edges.get(&edge_id) {
                Some((edge_id, NodeId::from(end_id)))
            } else {
                None
            }
        });
        let ys = (0..self.nodes.len()).filter_map(|begin_id| {
            if begin_id == id {
                None
            } else {
                let edge_id = EdgeId::from((begin_id, id));
                if let Some(_) = self.edges.get(&edge_id) {
                    Some((edge_id, NodeId::from(begin_id)))
                } else {
                    None
                }
            }
        });
        xs.chain(ys).collect()
    }

    #[allow(dead_code)]
    pub fn size_nodes(&self) -> usize {
        self.nodes.len()
    }
    #[allow(dead_code)]
    pub fn size_edges(&self) -> usize {
        self.edges.len()
    }

    #[allow(dead_code)]
    pub fn dump(&self) -> String
    where
        N: Debug,
        E: Debug,
    {
        let mut result = String::new();
        writeln!(result, "#Nodes").unwrap();
        for (index, value) in self.nodes.iter().enumerate() {
            writeln!(result, "{}:{:?}", index, value).unwrap();
        }
        writeln!(result, "#Edges").unwrap();
        for (key, value) in self.edges.iter() {
            writeln!(
                result,
                "{}({:?})->{}({:?}):{:?}",
                key.begin,
                self[key.begin()],
                key.end,
                self[key.end()],
                value
            )
            .unwrap();
        }
        result
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
