
//よく使うがライブラリにない関数をここに入れる

#[allow(dead_code)]
fn max<F>(a: F, b: F) -> F
where
    F: Ord,
{
    if a < b {
        b
    } else {
        a
    }
}

#[allow(dead_code)]
fn min<F>(a: F, b: F) -> F
where
    F: Ord,
{
    if a > b {
        b
    } else {
        a
    }
}



