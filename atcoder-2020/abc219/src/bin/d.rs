use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        ab: [(i64, i64); n],
    }
    let mut m = vec![vec![vec![std::i64::MAX; y + 1]; x + 1]; n + 1];
    m[0][0][0] = 0;
    for (i, &(a, b)) in ab.iter().enumerate() {
        for x0 in 0..=x {
            for y0 in 0..=y {
                let c = m[i][x0][y0];
                if c == std::i64::MAX {
                    continue;
                }
                let x1 = (x0 + a as usize).min(x);
                let y1 = (y0 + b as usize).min(y);
                m[i + 1][x0][y0] = m[i + 1][x0][y0].min(c);
                m[i + 1][x1][y1] = m[i + 1][x1][y1].min(c + 1);
            }
        }
    }
    let mut result = m[n][x][y];
    if result == std::i64::MAX {
        result = -1;
    };
    println!("{}", result);
}
