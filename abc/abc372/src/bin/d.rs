use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }

    let mut cum = vec![0i64; n + 1];
    let mut v = vec![]; // 最小値と位置の組
    for (i, &x) in h.iter().enumerate() {
        let j = v.partition_point(|&(x0, _)| x0 > x);
        v.resize(j, (0, 0)); // (0, 0): dummy
        if j == 0 {
            cum[1] += 1;
        } else {
            cum[v[j - 1].1 + 1] += 1;
        }
        v.push((x, i));
        cum[i + 1] -= 1;
    }

    for i in 0..n {
        cum[i + 1] += cum[i];
    }
    println!("{}", cum[1..].iter().join(" "));
}
