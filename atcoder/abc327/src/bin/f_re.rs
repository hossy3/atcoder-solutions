use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        w: usize,
        tx: [(usize, usize); n],
    }

    let t_max = *tx.iter().map(|(t, _)| t).max().unwrap_or(&1);
    let x_max = *tx.iter().map(|(_, x)| x).max().unwrap_or(&1);
    let mut m = vec![vec![0i32; x_max]; t_max];
    for &(t, x) in &tx {
        let (t0, t1) = (t - 1, t + d - 1);
        let (x0, x1) = (x - 1, x + w - 1);
        m[t0][x0] += 1;
        if t1 < t_max {
            m[t1][x0] -= 1;
        }
        if x1 < x_max {
            m[t0][x1] -= 1;
        }
        if t1 < t_max && x1 < x_max {
            m[t1][x1] += 1;
        }
    }

    for i in 0..t_max {
        for j in 0..(x_max - 1) {
            m[i][j + 1] += m[i][j];
        }
    }
    for i in 0..(t_max - 1) {
        for j in 0..x_max {
            m[i + 1][j] += m[i][j];
        }
    }

    let result = m
        .iter()
        .map(|v| v.iter().max().unwrap_or(&0))
        .max()
        .unwrap_or(&0);
    println!("{result}");
}
