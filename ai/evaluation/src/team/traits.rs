use glm::*;
pub trait TeamEvaluater {
    fn evaluate(&self, rights: &[Vec2], lefts: &[Vec2], balls: &[Vec2]) -> (f32, f32);
}
