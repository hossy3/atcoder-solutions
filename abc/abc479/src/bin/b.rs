use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }
    let mut cur = 0usize;
    let mut max = 0usize;
    for c in s {
        match c {
            'S' => {
                cur = 0;
            }
            'R' => {
                cur += 1;
                max = max.max(cur);
            }
            _ => unreachable!(),
        }
    }
    println!("{max}");
}
