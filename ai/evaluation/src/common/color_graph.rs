use super::Graph;
use glm::*;
use gnuplot::*;
use std::cell::*;
use std::collections::*;
#[derive(Clone, Debug, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum Color {
    White,
    Blue,
    Yellow,
}

#[allow(dead_code)]
pub type ColorGraph = Graph<(Vec2, Color), f32>;

impl ColorGraph {
    #[allow(dead_code)]
    pub fn show(&self) {
        let mut figure = Figure::new();
        let axes2d = figure.axes2d();
        //点の描画
        let mut white_points = vec![];
        let mut blue_points = vec![];
        let mut yellow_points = vec![];
        for (point, color) in self.get_nodes().iter() {
            match color {
                Color::White => white_points.push(point),
                Color::Blue => blue_points.push(point),
                Color::Yellow => yellow_points.push(point),
            }
        }
        if white_points.len() > 0 {
            axes2d.points(
                white_points.iter().map(|p| p.x),
                white_points.iter().map(|p| p.y),
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
