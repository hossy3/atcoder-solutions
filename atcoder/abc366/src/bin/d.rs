use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[[i64; n]; n]; n],
        q: usize,
        lr3: [(usize, usize, usize, usize, usize, usize); q],
    }

    let mut cum = vec![vec![vec![0i64; n + 1]; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                cum[i + 1][j + 1][k + 1] = a[i][j][k];
            }
        }
    }

    // 3次元累積和
    for i in 0..n {
        for j in 0..(n + 1) {
            for k in 0..(n + 1) {
                cum[i + 1][j][k] += cum[i][j][k];
            }
        }
    }
    for i in 0..(n + 1) {
        for j in 0..n {
            for k in 0..(n + 1) {
                cum[i][j + 1][k] += cum[i][j][k];
            }
        }
    }
    for i in 0..(n + 1) {
        for j in 0..(n + 1) {
            for k in 0..n {
                cum[i][j][k + 1] += cum[i][j][k];
            }
        }
    }

    for (lx, rx, ly, ry, lz, rz) in lr3 {
        let result = cum[rx][ry][rz]
            - (cum[rx][ry][lz - 1] + cum[lx - 1][ry][rz] + cum[rx][ly - 1][rz])
            + (cum[rx][ly - 1][lz - 1] + cum[lx - 1][ry][lz - 1] + cum[lx - 1][ly - 1][rz])
            - cum[lx - 1][ly - 1][lz - 1];
        println!("{result}");
    }
}
