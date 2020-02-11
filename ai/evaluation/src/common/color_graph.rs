use super::{EdgeId, Graph, NodeId};
use glm::*;
use gnuplot::*;
use std::cell::*;
use std::collections::*;
#[derive(Clone, Debug, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum Color {
    Black,
    Blue,
    Yellow,
}

#[allow(dead_code)]
pub type ColorGraph = Graph<(Vec2, Color), f32>;

impl ColorGraph {
    #[allow(dead_code)]
    pub fn add_with_color(&mut self, point: Vec2, color: Color) -> NodeId {
        self.add_node((point, color))
    }

    #[allow(dead_code)]
    pub fn connect_filter_map<F: Fn(&[(Vec2, Color)], usize, usize) -> Option<f32>>(
        &mut self,
        filter: F,
    ) {
        let nodes = self.get_nodes().to_vec();
        let size = self.size_nodes();
        //O(n^2)　すべての取りうるノードを検査する
        for (i, j) in (0..size)
            .map(|i| (0..i).chain(i + 1..size).map(move |j| (i, j)))
            .flatten()
        {
            if let Some(edge) = filter(&nodes, i, j) {
                self.add_edge((NodeId::from(i), NodeId::from(j)), edge);
            }
        }
    }

    #[allow(dead_code)]
    pub fn show(&self) {
        let mut figure = Figure::new();
        let axes2d = figure.axes2d();
        //点の描画
        let mut black_points = vec![];
        let mut blue_points = vec![];
        let mut yellow_points = vec![];
        for (point, color) in self.get_nodes().iter() {
            match color {
                Color::Black => black_points.push(point),
                Color::Blue => blue_points.push(point),
                Color::Yellow => yellow_points.push(point),
            }
        }
        if black_points.len() > 0 {
            axes2d.points(
                black_points.iter().map(|p| p.x),
                black_points.iter().map(|p| p.y),
                &[], //&[PlotOption::Color("white")],
            );
        }
        if blue_points.len() > 0 {
            axes2d.points(
                blue_points.iter().map(|p| p.x),
                blue_points.iter().map(|p| p.y),
                &[], //&[PlotOption::Color("blue")],
            );
        }
        if yellow_points.len() > 0 {
            axes2d.points(
                yellow_points.iter().map(|p| p.x),
                yellow_points.iter().map(|p| p.y),
                &[], //&[PlotOption::Color("yellow")],
            );
        }
        //線の描画
        if self.size_edges() > 0 {
            for (edge, weight) in self.get_edges().iter() {
                let (begin, _) = self[edge.begin()];
                let (end, _) = self[edge.end()];
                axes2d.arrow(
                    Coordinate::Axis(begin.x as f64),
                    Coordinate::Axis(begin.y as f64),
                    Coordinate::Axis(end.x as f64),
                    Coordinate::Axis(end.y as f64),
                    &[],
                );
            }
        }
        figure.show().unwrap();
    }
}
