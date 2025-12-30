use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Usize1; n],
        tb: [(usize, Usize1); q],
    }

    let mut v = vec![vec![(0usize, 0usize); n]; 32]; // (次の場所, 水の両)
    for i in 0..n {
        v[0][i] = (a[i], i + 1);
    }
    for i in 0..31 {
        for j in 0..n {
            v[i + 1][j] = (v[i][v[i][j].0].0, v[i][j].1 + v[i][v[i][j].0].1);
        }
    }

    for &(mut t, mut b) in &tb {
        let mut result = 0;
        for i in (0..32).rev() {
            let k = 1 << i;
            if t >= k {
                result += v[i][b].1;
                b = v[i][b].0;
                t -= k;
            }
        }
        println!("{result}");
    }
}
