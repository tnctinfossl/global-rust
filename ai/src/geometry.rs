use glm::*;
use std::f32::*;
//クロス積?を求める
#[inline(always)]
fn cross2(a: Vec2, b: Vec2) -> f32 {
    //分解
    let unpack = |p: Vec2| (p.x, p.y);
    let (x0, y0) = unpack(a);
    let (x1, y1) = unpack(b);
    //計算
    x0 * y1 - x1 * y0
}

#[test]
fn test_cross2() {
    assert_eq!(cross2(vec2(1.0, 0.0), vec2(0.0, 1.0)), 1.0);
    assert_eq!(cross2(vec2(0.0, 1.0), vec2(1.0, 0.0)), -1.0);
}

//90度まわす]
#[inline(always)]
fn flip2(p: Vec2) -> Vec2 {
    vec2(-p.y, p.x)
}

#[test]
fn test_flip() {
    assert_eq!(flip2(vec2(1.0, 0.0)), vec2(0.0, 1.0));
    assert_eq!(flip2(vec2(0.0, 1.0)), vec2(-1.0, 0.0));
}

#[inline(always)]
fn length2<S: BaseFloat, T: GenFloatVec<S>>(x: T) -> S {
    dot(x, x)
}

//二点a,bを通る直線と点pの距離
#[allow(dead_code)]
pub fn distance_line_point((a, b): (Vec2, Vec2), p: Vec2) -> f32 {
    //導出
    abs(dot(b - a, flip2(p)) + cross2(a, b)) / length(b - a)
}

#[test]
fn test_distance_line_point() {
    let d = distance_line_point((vec2(0.0, 0.0), vec2(0.0, 0.2)), vec2(1.0, 1.0));
    assert_eq!(d, 1.0);
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
