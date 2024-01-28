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
    degs.sort_by(|a, b| a.partial_cmp(b).unwrap());

    calc_score(&degs)
}

fn calc_score(degs: &[f64]) -> f64 {
    let n = degs.len();
    let mut score = 0f64;
    for &deg in degs {
        let i = degs.partition_point(|&x| x - deg <= 180.0);
        let deg0 = degs[(i + n - 1) % n] - deg;
        score = score.max(deg0.min(360.0 - deg0));

        let deg0 = degs[i % n] - deg;
        score = score.max(deg0.min(360.0 - deg0));
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
