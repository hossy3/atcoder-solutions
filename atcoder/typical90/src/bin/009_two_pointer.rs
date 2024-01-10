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

// two pointer
fn calc_score(degs: &[f64]) -> f64 {
    let n = degs.len();
    let mut score = 0f64;
    let mut j = 0;
    for i in 0..n {
        while j < n {
            let s = degs[j] - degs[i];
            let s0 = s.min(360.0 - s);
            score = score.max(s0);
            if s > 180.0 {
                break;
            }
            j += 1;
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
