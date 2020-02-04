use glm::*;
use std::ops::*;

#[derive(Debug, Clone)]
pub struct Graph {
    points: Vec<Vec2>,
    mattrix: Vec<Vec<Option<f32>>>,
}

impl Index<usize> for Graph {
    type Output = Vec2;
    fn index(&self, index: usize) -> &Vec2 {
        &self.points[index]
    }
}

impl Graph {
    #[allow(dead_code)]
    pub fn new(points: Vec<Vec2>) -> Graph {
        let size = points.len();
        let mut mattrix = vec![vec![None; size]; size];
        let d = distance(points[0], points[1]);
        mattrix[0][1] = Some(d);
        mattrix[1][0] = Some(d);
        for (index, &point) in points.iter().enumerate().skip(2) {
            //一番近い点を求める
            let (near_index, &near_point) = points
                .iter()
                .enumerate()
                .take(index)
                .min_by(|(_, &p), (_, &q)| {
                    use std::cmp::Ordering::*;
                    if distance(point, p) > distance(point, q) {
                        return Greater;
                    }
                    Less
                })
                .unwrap();
            let d = distance(point, near_point);
            mattrix[index][near_index] = Some(d);
            mattrix[near_index][index] = Some(d);
        }

        Graph {
            points: points,
            mattrix: mattrix,
        }
    }

    /*
      #[allow(dead_code)]
      pub fn insert_points(&mut self, points: &[Vec2]) -> Vec<usize> {
          let mut result = Vec::with_capacity(points.len());
          self.points.reserve(points.len());
          for point in points {
              result.push(self.points.len());
              self.points.push(*point);
          }
          result
      }

      #[allow(dead_code)]
      pub fn insert_lines(&mut self, lines: &[(usize, usize)]) {
          self.lines.reserve(lines.len());
          for line in lines {
              self.lines.push(*line);
          }
      }
    */

    #[allow(dead_code)]
    pub fn show(&self) {
        use gnuplot::*;
        let mut figure = Figure::new();
        let mut axe2d = figure.axes2d();
        //point
        let xs = self.points.iter().map(|p| p.x);
        let ys = self.points.iter().map(|p| p.y);
        axe2d.points(xs, ys, &[]);
        //arrows
        for begin in 0..self.points.len() {
            for end in begin + 1..self.points.len() {
                if let Some(_) = self.mattrix[begin][end] {
                    let xs_begin = Coordinate::Axis(self.points[begin].x as f64);
                    let ys_begin = Coordinate::Axis(self.points[begin].y as f64);
                    let xs_end = Coordinate::Axis(self.points[end].x as f64);
                    let ys_end = Coordinate::Axis(self.points[end].y as f64);
                    axe2d.arrow(
                        xs_begin,
                        ys_begin,
                        xs_end,
                        ys_end,
                        &[PlotOption::ArrowType(ArrowheadType::NoArrow)],
                    );
                }
            }
        }

        //show
        figure.show().unwrap();
    }
}
