use super::*;
use glm::*;
use rand::*;
use serde_derive::*;
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Field {
    pub infield: Vec2,
    pub outfield: Vec2,
    pub penalty_area_width: f32,
    pub penalty_area_depth: f32,
}

impl Default for Field {
    fn default() -> Field {
        //適当な値で初期化している[m]
        Field {
            infield: vec2(1.0, 1.0),
            outfield: vec2(1.1, 1.1),
            penalty_area_width: 0.5,
            penalty_area_depth: 0.2,
        }
    }
}

impl Field {
    #[allow(dead_code)]
    pub fn new(
        infield: Vec2,
        outfield: Vec2,
        penalty_area_width: f32,
        penalty_area_depth: f32,
    ) -> Field {
        Field {
            infield: infield,
            outfield: outfield,
            penalty_area_width: penalty_area_width,
            penalty_area_depth: penalty_area_depth,
        }
    }

    #[allow(dead_code)]
    pub fn ramdon_scene<R: Rng + ?Sized>(
        &self,
        random: &mut R,
        blues: u32,
        yellows: u32,
        ball: bool,
    ) -> Scene {
        //Scene::default()

        let random_robot = |r: &mut R| -> Robot {
            Robot::new(
                vec2rad(
                    r.gen_range(-self.infield.x / 2.0, self.infield.x / 2.0),
                    r.gen_range(-self.infield.y / 2.0, self.infield.y / 2.0),
                    r.gen_range(0.0, 2.0 * std::f32::consts::PI),
                ),
                robot::DIAMETOR_ROBOT,
            )
        };

        let random_ball = |r: &mut R| -> Ball {
            Ball::new(vec2(
                r.gen_range(-self.infield.x / 2.0, self.infield.x / 2.0),
                r.gen_range(-self.infield.y / 2.0, self.infield.y / 2.0),
            ))
        };

        let robots = (0..blues)
            .map(|id| RobotID::Blue(id))
            .chain((0..yellows).map(|id| RobotID::Yellow(id)))
            .map(|id| (id, random_robot(random)))
            .collect();
        let ball = if ball {
            Some(random_ball(random))
        } else {
            None
        };
        Scene {
            ball: ball,
            robots: robots,
        }
    }

    //枝刈りメソッド
    #[allow(dead_code)]
    pub fn prune<'a>(&self, scene: &'a Scene) -> Option<&'a Scene> {
        if !scene.robots.values().all(|r: &Robot| self.overlap(r)) {
            return None;
        }
        if !scene.ball.iter().all(|b: &Ball| self.overlap(b)) {
            return None;
        } else {
            return Some(scene);
        }
    }
}

impl Overlap<Robot> for Field {
    fn overlap(&self, rhs: &Robot) -> bool {
        let infield = self.infield / 2.0;
        let robot_abs = rhs.position.to_vec2().abs();
        infield.x >= robot_abs.x && infield.y >= robot_abs.y
    }
}

impl Overlap<Ball> for Field {
    fn overlap(&self, rhs: &Ball) -> bool {
        let infield = self.infield / 2.0;
        let ball_abs = rhs.position.abs();
        infield.x >= ball_abs.x && infield.y >= ball_abs.y
    }
}
