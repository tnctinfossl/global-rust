use crate::common::*;

use glm::*;

#[derive(Debug)]
pub struct LinkDomination {
    field: Rectangle,
    goals: (Vec2, Vec2),
}

impl LinkDomination {
    #[allow(dead_code)]
    pub fn new(field: Rectangle, goals: (Vec2, Vec2)) -> LinkDomination {
        LinkDomination {
            field: field,
            goals: goals,
        }
    }

    pub fn evaluate(&self, rights: &[Vec2], lefts: &[Vec2]) -> (f32, f32) {
        (0.0, 0.0)
    }
}
