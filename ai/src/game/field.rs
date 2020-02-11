use super::*;
use glm::*;
use rand::*;
use serde_derive::*;
use std::collections::BTreeMap;
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Field {
    pub infield: Vec2,
    pub outfield: Vec2,
    pub penalty_area_width: f32,
    pub penalty_area_depth: f32,
}

impl Default for Field {
    fn default() -> Field {
        //適当な値で初期化している[mm]
        Field {
            infield: vec2(12000.0, 9000.0),
            outfield: vec2(13400.0, 10400.0),
            penalty_area_width: 2400.0,
            penalty_area_depth: 1200.0,
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
        right: u32,
        left: u32,
        ball_flag: bool,
    ) -> Scene {
        //Scene::default()

        let random_robot = |r: &mut R| -> Robot {
            Robot::new(
                vec2rad(
                    r.gen_range(-self.infield.x / 2.0 * 0.8, self.infield.x / 2.0 * 0.8),
                    r.gen_range(-self.infield.y / 2.0 * 0.8, self.infield.y / 2.0 * 0.8),
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

        let rights: BTreeMap<RobotID, Robot> = (0..right)
            .map(|id| (RobotID::Blue(id), random_robot(random)))
            .collect();
        let lefts: BTreeMap<RobotID, Robot> = (0..left)
            .map(|id| (RobotID::Yellow(id), random_robot(random)))
            .collect();
        let mut ball = None;
        if ball_flag {
            ball = Some(random_ball(random));
        }
        Scene {
            rights: rights,
            lefts: lefts,
            ball: ball,
        }
    }

    //枝刈りメソッド
    #[allow(dead_code)]
    pub fn prune(&self, scene: Scene) -> Option<Scene> {
        if !scene.rights.values().all(|r: &Robot| self.overlap(*r)) {
            return None;
        }
        if !scene.lefts.values().all(|r: &Robot| self.overlap(*r)) {
            return None;
        }
        if !scene.ball.iter().all(|b: &Ball| self.overlap(*b)) {
            return None;
        } else {
            return Some(scene);
        }
    }
}

impl<T> Overlap<T> for Field
where
    T: Into<Circle>,
{
    fn overlap(&self, rhs: T) -> bool {
        let circle = rhs.into();
        let infield = self.infield / 2.0;
        let check_x = -infield.x < circle.center.x && circle.center.x < infield.x;
        let check_y = -infield.y < circle.center.y && circle.center.y < infield.y;
        check_x && check_y
    }
}
