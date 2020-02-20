use super::*;
use glm::*;
use serde_derive::*;
const BALL_RADIUS: f32 = 0.02; //TODO 適当な値です。
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Ball {
    pub position: Vec2,
}

impl Default for Ball {
    #[allow(dead_code)]
    fn default() -> Ball {
        //デフォルト値のインスタンス生成
        Ball {
            position: vec2(0.0, 0.0),
        }
    }
}

impl Ball {
    #[allow(dead_code)]
    pub fn new(position: Vec2) -> Ball {
        //インスタンス生成
        Ball { position: position }
    }
}

impl From<Ball> for Circle {
    fn from(ball: Ball) -> Self {
        //ball->Circle
        Circle::new(ball.position, BALL_RADIUS)
    }
}
