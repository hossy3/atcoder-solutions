use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; q],
    }

    let mut v = vec![vec![]; n.max(q + 1)];
    let mut results = vec![];
    'outer: for &x0 in &x {
        if x0 == 0 {
            for i in 0..=q {
                for j in 0..n {
                    if v[j].len() == i {
                        v[j].push(x0);
                        results.push(j + 1);
                        continue 'outer;
                    }
                }
            }
        } else {
            v[x0 - 1].push(x0);
            results.push(x0);
        }
    }

    println!("{}", results.iter().join(" "));
}
