use proconio::input;

fn f(i: usize, xy: &[(f64, f64)]) -> f64 {
    let n = xy.len();
    let mut degs = Vec::with_capacity(n - 1);
    for j in 0..n {
        if i == j {
            continue;
        }
        let rad = (xy[j].1 - xy[i].1).atan2(xy[j].0 - xy[i].0);
        let deg = rad.to_degrees();
        degs.push(deg);
    }

    calc_score(&degs)
}

fn calc_score(degs: &[f64]) -> f64 {
    let n = degs.len();
    let mut score = 0f64;
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let deg = (degs[j] - degs[i]).rem_euclid(360.0);
            score = score.max(deg.min(360.0 - deg));
        }
    }
    score
}

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    };
    let score = (0..n).fold(0f64, |acc, i| acc.max(f(i, &xy)));
    println!("{score}");
}
