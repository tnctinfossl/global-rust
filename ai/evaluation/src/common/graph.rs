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
    pub fn new(points: &[Vec2]) -> Graph {
        let size = points.len();
        let mut mattrix = vec![vec![None; size]; size];

        let mut result = Graph {
            points: points.to_vec(),
            mattrix: mattrix,
        };
        result.connect(0, 1);
        result.connect(0, 2);
        result.connect(1, 2);

        for (index, point) in points.iter().enumerate().skip(3) {
            result.insert(index, *point);
        }
        result
    }

    fn connect(&mut self, i: usize, j: usize) {
        let d = distance(self.points[i], self.points[j]);
        self.mattrix[i][j] = Some(d);
        self.mattrix[j][i] = Some(d);
    }

    fn connected(&self, i: usize) -> Vec<usize> {
        let mut result = vec![];
        for (index, value) in self.mattrix[i].iter().enumerate() {
            if let Some(_) = value {
                result.push(index);
            }
        }
        result
    }

    fn insert(&mut self, index: usize, point: Vec2) {
        //一番近い点を求める
        let (nearest_index, _) = self
            .points
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
        self.connect(index, nearest_index);
        /*for near_index in self.connected(nearest_index) {
            self.connect(index, near_index);
        }*/
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
