use crate::common::*;
use bmp::*;
use glm::*;
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
        let right: f32 = rights.iter().map(|q| exp(-distance2(point, *q))).sum();
        let left: f32 = lefts.iter().map(|q| exp(-distance2(point, *q))).sum();
        right - left
    }

    #[allow(dead_code)]
    pub fn save(&self, filename: &str, rights: &[Vec2], lefts: &[Vec2]) {
        //計算
        let x0 = self.rect.x0();
        let y0 = self.rect.y0();
        let dx = self.rect.sx() / self.n as f32;
        let dy = self.rect.sy() / self.m as f32;

        let mut img = Image::new(self.n as u32 + 1, self.m as u32 + 1);
        //着色アルゴリズム
        let color = |value: f32| {
            //色
            let red = vec3(255.0, 0.0, 0.0);
            let blue = vec3(0.0, 0.0, 255.0);
            let color = if value > 0.0 {
                red * (1.0 - exp(-value))
            } else {
                blue * (1.0 - exp(value))
            };
            Pixel::new(color.x as u8, color.y as u8, color.z as u8)
        };

        let ij_xy = |i, j| {
            let x = x0 + dx * i as f32;
            let y = y0 + dy * j as f32;
            vec2(x, y)
        };
        let mut sum = 0.0;
        let begin = Instant::now();
        for (i, j) in img.coordinates() {
            let z = self.field(ij_xy(i, j), rights, lefts);
            img.set_pixel(i, j, color(z));
            sum += z;
        }
        let end = Instant::now();
        println!("sum={}", sum / ((self.n + 1) * (self.m + 1)) as f32);
        println!("time={:?}", end - begin);
        let xy_ij = |x, y| {
            let x = (x - x0) / self.rect.sx() * self.n as f32;
            let y = (y - y0) / self.rect.sy() * self.m as f32;
            (x as u32, y as u32)
        };

        for right in rights {
            let ij = xy_ij(right.x, right.y);
            img.set_pixel(ij.0, ij.1, px!(255, 0, 255));
        }

        for left in lefts {
            let ij = xy_ij(left.x, left.y);
            img.set_pixel(ij.0, ij.1, px!(0, 255, 255));
        }
        img.save(filename).unwrap();
    }

    pub fn evaluate(&self, rights: &[Vec2], lefts: &[Vec2], _balls: &[Vec2]) -> (f32, f32) {
        //計算
        let x0 = self.rect.x0();
        let y0 = self.rect.y0();
        let dx = self.rect.sx() / self.n as f32;
        let dy = self.rect.sy() / self.m as f32;
        let ij_xy = |i, j| {
            let x = x0 + dx * i as f32;
            let y = y0 + dy * j as f32;
            vec2(x, y)
        };
        //let mut sum = 0.0;
        let area = ((self.n + 1) * (self.m + 1)) as f32;
        let sum: f32 = (0..self.n + 1)
            .into_iter()
            .map(|i: usize| {
                let s: f32 = (0..self.m + 1)
                    .into_iter()
                    .map(|j: usize| self.field(ij_xy(i, j), rights, lefts))
                    .sum();
                s
            })
            .sum();
        (sum / area, -sum / area)
    }
}
