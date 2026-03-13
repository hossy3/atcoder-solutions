use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        s: Usize1,
        mut q: usize,
        x: [isize; n],
    }

    let x0 = x
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, i))
        .sorted()
        .collect::<Vec<_>>();

    let mut i = x0.iter().find_position(|&&(_, s0)| s == s0).unwrap().0;
    let mut v = vec![false; n];
    v[i] = true;

    while q > 0 {
        let j = if i == 0 {
            i + 1
        } else if i == n - 1 {
            i - 1
        } else {
            let d0 = x0[i].0 - x0[i - 1].0;
            let d1 = x0[i + 1].0 - x0[i].0;
            if d0 < d1 || (d0 == d1 && x0[i - 1].1 < x0[i + 1].1) {
                i - 1
            } else {
                i + 1
            }
        };
        q -= 1;
        if v[j] {
            q = q % 2; // 最後は往復運動になるはず
        }
        v[j] = true;
        i = j;
    }

    let result = x0[i].1 + 1;
    println!("{result}");
}
