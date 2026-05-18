use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut max = 0;
    let mut cur = 0;
    for &c in &s {
        if c == '(' {
            cur += 1;
        } else {
            cur -= 1;
        }
        max = max.max(cur);
    }
    println!("{max}");
}
