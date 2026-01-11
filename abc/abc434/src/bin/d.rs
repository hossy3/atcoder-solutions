use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        udlr: [(Usize1, Usize1, Usize1, Usize1); n],
    }

    // 雲で隠れた数を逆順で調べる
    const N: usize = 2000;
    let mut imos = vec![vec![0isize; N + 1]; N + 1];
    for &(u, d, l, r) in udlr.iter().rev() {
        imos[u][l] += 1;
        imos[u][r + 1] -= 1;
        imos[d + 1][l] -= 1;
        imos[d + 1][r + 1] += 1;
    }

    for y in 0..(N + 1) {
        for x in 0..N {
            imos[y][x + 1] += imos[y][x];
        }
    }
    for y in 0..N {
        for x in 0..(N + 1) {
            imos[y + 1][x] += imos[y][x];
        }
    }

    let mut all = N * N;
    let mut m = vec![vec![0isize; N + 1]; N + 1];
    for i in 0..N {
        for j in 0..N {
            if imos[i][j] > 0 {
                all -= 1;
            }
            if imos[i][j] == 1 {
                m[i + 1][j + 1] += 1;
            }
        }
    }

    for y in 0..(N + 1) {
        for x in 0..N {
            m[y][x + 1] += m[y][x];
        }
    }
    for y in 0..N {
        for x in 0..(N + 1) {
            m[y + 1][x] += m[y][x];
        }
    }

    for &(u, d, l, r) in &udlr {
        let result = all + (m[d + 1][r + 1] + m[u][l] - (m[d + 1][l] + m[u][r + 1])) as usize;
        println!("{result}");
    }
}
