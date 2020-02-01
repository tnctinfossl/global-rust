use glm::*;
use std::collections::HashMap;

struct Node<V, T> {
    pub point: V,
    pub data: T,
}

impl<V, T> Node<V, T> {
    #[allow(dead_code)]
    fn new(point: V, data: T) -> Node<V, T> {
        Node {
            point: point,
            data: data,
        }
    }
}

#[allow(dead_code)]
pub struct Canvas<V, T> {
    nodes: Vec<Node<V, T>>,
    triangles: Vec<(usize, usize, usize)>,
}

impl<V, T> Canvas<V, T>
where
    V: GenFloatVec<f32>,
{
    #[allow(dead_code)]
    pub fn new() -> Canvas<T, V> {
        Canvas {
            nodes: vec![],
            triangles: vec![],
        }
    }
    #[allow(dead_code)]
    pub fn insert(&mut self, point: V, data: T) {
        //要素の追加と添字の取得
        let node = Node::new(point, data);
        self.nodes.push(node);
        let index = self.nodes.len() - 1;
        //配置
        if index < 2 {
            return;
        }
    }
}
