use glm::*;

//クロス積?を求める
#[inline(always)]
pub fn cross2d(a: Vec2, b: Vec2) -> f32 {
    //分解
    let unpack = |p: Vec2| (p.x, p.y);
    let (x0, y0) = unpack(a);
    let (x1, y1) = unpack(b);
    //計算
    x0 * y1 - x1 * y0
}

//90度まわす つまり法線を求める
#[inline(always)]
pub fn turn_right(p: Vec2) -> Vec2 {
    vec2(-p.y, p.x)
}

#[inline(always)]
fn length2<S: BaseFloat, T: GenFloatVec<S>>(x: T) -> S {
    dot(x, x)
}

//二点a,bを通る直線と点pの距離
#[allow(dead_code)]
pub fn distance_line_point((a, b): (Vec2, Vec2), p: Vec2) -> f32 {
    //導出
    abs(dot(b - a, turn_right(p)) + cross2d(a, b)) / length(b - a)
}

//二点a,bを通る線分と点pの距離
#[allow(dead_code)]
pub fn distance_segment_point((a, b): (Vec2, Vec2), p: Vec2) -> f32 {
    let tt = -dot(b - a, a - p) / length2(a - b);
    if tt < 0.0 {
        return distance(p, a);
    }
    if tt > 1.0 {
        return distance(p, b);
    }
    return distance_line_point((a, b), p);
}

//線分[src,dest]と最も近いものとの距離を求める
#[allow(dead_code)]
pub fn distance_segment_nearest_points<I>((src, dest): (Vec2, Vec2), points: I) -> Option<f32>
where
    I: Iterator<Item = Vec2>,
{
    points
        .map(|point| -> f32 { distance_segment_point((src, dest), point) })
        //.reduce(|a, b| if a > b { b } else { a })
        .min_by(|a, b| a.partial_cmp(b).unwrap())
}

//パスの経路[begin,end]に最も近い場所との距離を求める。
//この際、[,begin],[end,]が最も近い場所は含まない。
//制約 objects にはbegin,endが含まれていないこと
#[allow(dead_code)]
pub fn distance_path_nearest_points<'a, I: Iterator<Item = &'a Vec2>>(
    (begin, end): (Vec2, Vec2),
    objects: I,
) -> Option<f32> {
    /*  座標beginを通り、線分[begin,end]に垂直な直線fと
     ** 座標endを通り、線分[begin,end]に垂直な直線gに挟まれた領域に含まれる
     ** 座標郡objectsと線分[begin,end]の距離を求め、最短のものを返す
     */

    //[begin,end]を通る直線は ax+by+c=0を満たす
    let motion = vec2(1.0, -1.0) * (end - begin);
    let turn = turn_right(motion);

    //[begin,end]の法線かつbeginを通る直線fを求める
    let f = |p: Vec2| dot(turn, p) - dot(turn, begin);

    //[begin,end]の法線かつendを通る直線gを求める
    let g = |p: Vec2| dot(turn, p) - dot(turn, end);
    println!(
        "{},{},{},{}",
        turn.x,
        turn.y,
        -dot(turn, begin),
        -dot(turn, end)
    );
    objects
        .filter_map(|p: &Vec2| {
            if 0.0 <= f(*p) && g(*p) <= 0.0 {
                Some(distance_line_point((begin, end), *p))
            } else {
                None
            }
        })
        .min_by(|a, b| a.partial_cmp(b).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_turn_right() {
        assert_eq!(turn_right(vec2(1.0, 0.0)), vec2(0.0, 1.0));
        assert_eq!(turn_right(vec2(0.0, 1.0)), vec2(-1.0, 0.0));
    }
    #[test]
    fn test_distance_line_point() {
        let d = distance_line_point((vec2(0.0, 0.0), vec2(0.0, 0.2)), vec2(1.0, 1.0));
        assert_eq!(d, 1.0);
    }
    #[test]
    fn test_cross2() {
        assert_eq!(cross2d(vec2(1.0, 0.0), vec2(0.0, 1.0)), 1.0);
        assert_eq!(cross2d(vec2(0.0, 1.0), vec2(1.0, 0.0)), -1.0);
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
