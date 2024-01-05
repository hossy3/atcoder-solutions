use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
    }
    let x = x.iter().map(|&c| c as usize - '0' as usize).collect_vec();
    let yes = (x[0] == x[1] && x[1] == x[2] && x[2] == x[3])
        || (((x[0] + 1) % 10 == x[1]) && ((x[1] + 1) % 10 == x[2]) && ((x[2] + 1) % 10 == x[3]));
    println!("{}", if yes { "Weak" } else { "Strong" });
}
