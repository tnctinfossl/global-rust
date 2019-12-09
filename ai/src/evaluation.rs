extern crate model;

use model::*;
pub mod field;
pub mod pass;
pub mod geometry;
pub use field::*;

//パスの可能性を評価する
#[allow(dead_code)]
pub fn passable<'a, I: Iterator<Item = &'a Vec2>>((begin, end): (Vec2, Vec2), objects: I) -> f32 {
    let nearest_distance =
        distance_path_nearest_points((begin, end), objects).unwrap_or(std::f32::MAX);
    let path_distance = distance(begin, end);
    println!("{},{}", nearest_distance, path_distance);
    -log(nearest_distance + 1.0) - log(path_distance + 1.0)
}

//シュートの可能性を評価する
#[allow(dead_code)]
pub fn shootable(field: &Field, mine: &Team, yours: &Team) -> f32 {
    //計算量O(n2)程度
    //let goal = field.your_goal(mine);
    /*
      mine.robots
          .iter()
          .map(|target: &Box<Robot>| {
              let others = mine
                  .robots
                  .iter()
                  .filter(|cmp: &&Box<Robot>| cmp.id != target.id)
                  .chain(yours.robots.iter());
              let distance = others
                  .map(|other: &Box<Robot>| {
                      distance_segment_point((target.position, goal), other.position)
                  })
                  .reduce(util::min)
          })
    */
    0.0 //準備中
}

    #[test]
    fn test_distance_path_nearest_points() {
        let (begin, end) = (vec2(1.0, 1.0), vec2(3.0, 3.0));
        assert_eq!(distance_path_nearest_points((begin, end), [].iter()), None);
        assert_eq!(
            distance_path_nearest_points((begin, end), [vec2(2.0, 0.0)].iter()),
            Some(sqrt(2.0))
        );
        assert_eq!(
            distance_path_nearest_points((begin, end), [vec2(0.0, 2.0)].iter()),
            Some(sqrt(2.0))
        );
        assert_eq!(
            distance_path_nearest_points((begin, end), [vec2(0.0, 0.0)].iter()),
            None
        );
        assert_eq!(
            distance_path_nearest_points((end, begin), [vec2(2.0, 0.0)].iter()),
            Some(sqrt(2.0))
        );
        assert_eq!(
            distance_path_nearest_points((end, begin), [vec2(0.0, 2.0)].iter()),
            Some(sqrt(2.0))
        );
        assert_eq!(
            distance_path_nearest_points((end, begin), [vec2(0.0, 0.0)].iter()),
            None
        );
    }
}
