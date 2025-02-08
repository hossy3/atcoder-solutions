use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    }

    const N: usize = 1000;
    let mut v = vec![];

    for (i, &x) in p.iter().enumerate() {
        if v.len() == 0 {
            let mut v0 = Vec::with_capacity(N);
            v0.push(i + 1);
            v.push(v0);
            continue;
        }
        let mut x = x;

        let j_max = v.len();
        for j in 0..j_max {
            if x > v[j].len() {
                x -= v[j].len();
                continue;
            }
            v[j].insert(x, i + 1);
            if v[j].len() >= N {
                let mut v0 = Vec::with_capacity(N);
                for &y in &v[j][(N / 2)..] {
                    v0.push(y);
                }
                v.insert(j + 1, v0);
                v[j].resize(N / 2, 0);
            }
            break;
        }
    }

    let mut results: Vec<usize> = vec![];
    for v in &v {
        results.extend(v);
    }
    let result = results.iter().join(" ");
    println!("{result}");
}
