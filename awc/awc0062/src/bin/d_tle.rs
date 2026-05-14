use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut result = 0usize;
    for l in 0..(n - 1) {
        for r in (l + 1)..n {
            let mut diff = 0usize;
            for k in 0..(n - r) {
                if s[l + k] != s[r + k] {
                    diff += 1;
                    if diff == 2 {
                        break;
                    }
                }
                if diff == 1 {
                    result += 1;
                }
            }
        }
    }
    println!("{result}");
}
