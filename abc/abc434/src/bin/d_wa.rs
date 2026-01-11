use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        udlr: [(Usize1, Usize1, Usize1, Usize1); n],
    }

    // 雲で隠れた数を逆順で調べる
    const N: usize = 2000;
    const M: usize = 64;
    let mut v = vec![0];
    let mut m = vec![vec![0usize; N / M + 1]; 2000];

    for &(u, d, l, r) in udlr.iter().rev() {
        let mut diff = 0usize;
        for i in u..=d {
            if l / M == r / M {
                let mut x = 0usize;
                for j in l..=r {
                    x |= 1 << j;
                }
                let j = l / M;
                let x0 = m[i][j].count_ones() as usize;
                m[i][j] |= x;
                let x1 = m[i][j].count_ones() as usize;
                diff += x1 - x0;
                continue;
            }

            let mut x = 0usize;
            for j in l..N {
                x |= 1 << j;
            }
            let j = l / M;
            let x0 = m[i][j].count_ones() as usize;
            m[i][j] |= x;
            let x1 = m[i][j].count_ones() as usize;
            diff += x1 - x0;

            x = 0usize;
            for j in 0..=r {
                x |= 1 << j;
            }
            let j = r / M;
            let x0 = m[i][j].count_ones() as usize;
            m[i][j] |= x;
            let x1 = m[i][j].count_ones() as usize;
            diff += x1 - x0;

            for j in (l + 1)..r {
                let x0 = m[i][j].count_ones() as usize;
                m[i][j] = usize::MAX;
                let x1 = N;
                diff += x1 - x0;
            }
        }

        v.push(diff);
    }

    for x in v[1..].iter().rev() {
        let result = N * N - x;
        println!("{result}");
    }
}
