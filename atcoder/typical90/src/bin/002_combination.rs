use itertools::Itertools;
use proconio::input;

fn f(v: &[bool]) -> bool {
    let mut level = 0usize;
    for &b in v {
        if b {
            level += 1;
        } else {
            if level == 0 {
                return false;
            }
            level -= 1;
        }
    }

    level == 0
}

fn main() {
    input! {
        n: usize,
    }
    if n == 0 || n % 2 == 1 {
        return;
    }
    for v in (0..n).combinations(n / 2) {
        let mut v0 = vec![false; n];
        for i in v {
            v0[i] = true;
        }
        if !f(&v0) {
            continue;
        }
        let result = v0.iter().map(|&b| if b { '(' } else { ')' }).join("");
        println!("{result}");
    }
}
