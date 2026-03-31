use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    }

    let mut s = vec![];
    for _ in 0..n {
        input! {
            m: usize,
            s0: [usize; m],
        }
        s.push(s0);
    }

    input! {
        q: usize,
        vd: [(Usize1, Usize1); q],
    }
    let mut result = 0usize;
    for (v, d) in vd {
        if s[v][d] > 0 {
            s[v][d] -= 1;
        } else {
            result += 1;
        }
    }

    for s in s {
        println!("{}", s.iter().join(" "));
    }
    println!("{result}");
}
