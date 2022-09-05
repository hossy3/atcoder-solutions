use itertools::Itertools;
use proconio::{input, marker::Chars};

fn f(b: &[bool]) -> bool {
    if b[0] {
        return false;
    }
    let a = [b[6], b[3], b[1] || b[7], b[4], b[2] || b[8], b[5], b[9]];
    let b = (a[0] && !a[1] && (a[2] || a[3] || a[4] || a[5] || a[6]))
        || ((a[0] || a[1]) && !a[2] && (a[3] || a[4] || a[5] || a[6]))
        || ((a[0] || a[1] || a[2]) && !a[3] && (a[4] || a[5] || a[6]))
        || ((a[0] || a[1] || a[2] || a[3]) && !a[4] && (a[5] || a[6]))
        || ((a[0] || a[1] || a[2] || a[3] || a[4]) && !a[5] && a[6]);
    b
}

fn main() {
    input! {
        s: Chars,
    }
    let b = s.iter().map(|&c| c == '1').collect_vec();
    let yes = f(&b);
    println!("{}", if yes { "Yes" } else { "No" });
}
