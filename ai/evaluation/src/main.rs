mod common;
mod team;
use common::*;
use glm::*;
use std::time::Instant;
use team::*;
fn main() {
    use glm;
    use rand::*;
    let mut gen = rand::thread_rng();
    let field = Rectangle::new(vec2(-64.0, 50.0), vec2(128.0, -100.0));
    let mine: Vec<_> = (0..10).map(|_| field.sample(&mut gen)).collect();
    //let yours: Vec<_> = (0..10).map(|_| field.sample(&mut gen)).collect();
    let mut polygons = Graph::new(&mine);
    //let points =
    /*for point in mine {
        polygons.insert(point);
    }*/
    //polygons.insert_lines(&[(0, 1), (2, 1)]);
    polygons.show();
}
