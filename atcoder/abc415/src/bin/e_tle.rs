use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i64; w]; h],
        p: [i64; h + w - 1],
    }

    let mut v = vec![vec![i64::MAX; w]; h]; // コイン現在必要
    let x = (p[h + w - 2] - a[h - 1][w - 1]).max(0);
    v[h - 1][w - 1] = x;

    for i in (0..(h + w - 1)).rev() {
        for i0 in 0..h {
            let j0 = i as i64 - i0 as i64;
            if j0 < 0 {
                continue;
            }
            let j0 = j0 as usize;
            if j0 >= w {
                continue;
            }
            if i0 < h - 1 {
                let x0 = (p[i] + v[i0 + 1][j0] - a[i0][j0]).max(0);
                v[i0][j0] = x0.min(v[i0][j0]);
            }
            if j0 < w - 1 {
                let x0 = (p[i] + v[i0][j0 + 1] - a[i0][j0]).max(0);
                v[i0][j0] = x0.min(v[i0][j0]);
            }
        }
    }

    let result = v[0][0];
    println!("{result}");
}
