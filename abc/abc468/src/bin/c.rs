use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        q: [Usize1; n],
    }

    if p >= q {
        println!("0");
        return;
    }

    let mut b = false;
    let mut count = 0usize;
    for v in (0..n).permutations(n) {
        if b {
            if v == q {
                break;
            }
            count += 1;
        } else {
            if v == p {
                b = true;
            }
        }
    }
    println!("{count}");
}
