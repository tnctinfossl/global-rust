extern crate gnuplot;
use gnuplot::*;
extern crate nalgebra_glm;
mod object;

// 解きたい関数 f
fn f(x: f64) -> f64 {
    x * x - 2.0
}

// fの導関数
fn df(x: f64) -> f64 {
    2.0 * x
}

fn main() {
    //描画範囲
    let xrange = (-100., 100.);
    let yrange = (-100., 100.);
    let points: Vec<(f32, f32)> = (1..10)
        .map(|_| {
            (
                rand::random::<f32>() * (xrange.1 - xrange.0) + xrange.0,
                rand::random::<f32>() * (yrange.1 - yrange.0) + yrange.0,
            )
        })
        .collect();

    let mut fg = Figure::new();
    // x軸を表示
    let axes = fg.axes2d();
    axes.set_x_axis(true, &[]);
    axes.set_y_axis(true, &[]);

    let x: Vec<_> = points.iter().map(|v| v.0).collect();
    let y: Vec<_> = points.iter().map(|v| v.1).collect();

    axes.points(x.iter(), y.iter(), &[Color("blue"), PointSymbol('O')]);
    fg.show().unwrap();
    /*
    // 関数のプロットに使うx座標たち
    let mut t = vec![];
    for i in 0..100i32 {
        t.push(i as f64 * 0.1 - 5.0);
    }
    let mut fg = Figure::new();
    // axes2d()はmutable borrowを作るので後でshow()するには別スコープを作る必要がある
    {
        let axes = fg.axes2d();
        // x軸を表示
        axes.set_x_axis(true, &[]);
        // 表示範囲の指定
        axes.set_x_range(Fix(0.0), Fix(5.0));
        axes.set_y_range(Fix(-5.0), Fix(30.0));
        // fのプロット
        axes.lines(t.iter(), t.iter().map(|&x| f(x)), &[Color("red")]);

        // ニュートン法
        let mut x = 4.0; // 初期値
        while f(x).abs() > 1e-10 {
            // f(x)が十分小さくなるまで続ける
            // 関数値のプロット
            axes.points(&[x], &[f(x)], &[Color("blue"), PointSymbol('O')]);
            // 関数値に縦線を引く
            axes.lines(&[x, x], &[-5.0, 30.0], &[Color("blue")]);
            // 接線のプロット
            axes.lines(
                t.iter(),
                t.iter().map(|&p| df(x) * (p - x) + f(x)),
                &[Color("black")],
            );
            // 値の更新
            x = x - f(x) / df(x);
        }
        println!("solution: {}", x);
    }
    fg.show()?;*/
}
