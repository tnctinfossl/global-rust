use crate::common::*;
use glm::*;
use gnuplot::*;
use rayon::prelude::*;
use std::time::Instant;
//存在場

pub struct FieldDomination {
    rect: Rectangle,
    n: usize, //分割数(x)
    m: usize, //分割数(y)
}

fn distance2(p: Vec2, q: Vec2) -> f32 {
    let d = p - q;
    dot(d, d)
}

fn length2(p: Vec2) -> f32 {
    dot(p, p)
}

impl FieldDomination {
    #[allow(dead_code)]
    pub fn new(rect: Rectangle, (n, m): (usize, usize)) -> FieldDomination {
        FieldDomination {
            rect: rect,
            n: n,
            m: m,
        }
    }

    #[allow(dead_code)]
    fn field(&self, point: Vec2, rights: &[Vec2], lefts: &[Vec2]) -> f32 {
        let l2 = length2(self.rect.size);
        let f = |p: Vec2, q: Vec2| -> f32 {
            let s2 = distance2(p, q) / l2;
            1.0 / (s2 + 1.0)
        };
        let r = rights.iter().map(|r| f(point, *r));
        let l = lefts.iter().map(|l| -f(point, *l));
        let sum: f32 = r.chain(l).sum();
        sum
    }

    #[allow(dead_code)]
    pub fn show(&self, rights: &[Vec2], lefts: &[Vec2]) {
        //計算
        let x0 = self.rect.x0();
        let y0 = self.rect.y0();
        let dx = self.rect.sx() / self.n as f32;
        let dy = self.rect.sy() / self.m as f32;
        let indexs: Vec<_> = (0..self.n)
            .map(|i| (0..self.m).map(move |j| (i, j)))
            .flatten()
            .collect();
        let begin = Instant::now();
        let graph: Vec<Vec3> = indexs
            .par_iter()
            .map(|(i, j)| {
                let x = x0 + dx * (*j as f32);
                let y = y0 + dy * (*i as f32);
                (x, y)
            })
            .map(|(x, y)| vec3(x, y, self.field(vec2(x, y), &rights, &lefts)))
            .collect();
        let sum: f32 = graph.iter().map(|p| p.z).sum();
        let end = Instant::now();
        println!("{:?}", end - begin);
        println!("{}", sum / (self.n * self.m) as f32);
        //描画
        let mut figure = Figure::new();
        let axe3d = figure.axes3d();
        axe3d.set_view(0.0, 0.0);
        axe3d.surface(
            graph.iter().map(|p| p.z),
            self.n,
            self.m,
            Some((
                self.rect.x0() as f64,
                self.rect.y0() as f64,
                self.rect.x1() as f64,
                self.rect.y1() as f64,
            )),
            &[],
        );

        axe3d.points(
            rights.iter().map(|p| p.x as f64),
            rights.iter().map(|p| p.y as f64),
            Some(0.0).iter().cycle(),
            &[PlotOption::Color("orange")],
        );

        axe3d.points(
            lefts.iter().map(|p| p.x as f64),
            lefts.iter().map(|p| p.y as f64),
            Some(0.0).iter().cycle(),
            &[PlotOption::Color("blue")],
        );
        figure.show().unwrap();
    }
}
