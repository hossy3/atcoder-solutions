use proconio::{input, marker::Chars};

fn f(s: &[Vec<char>], h0: usize, h1: usize, w0: usize, w1: usize) -> bool {
    for h in h0..=h1 {
        for w in w0..=w1 {
            let x0 = s[h][w];
            let x1 = s[h0 + h1 - h][w0 + w1 - w];
            if x0 != x1 {
                return false;
            }
        }
    }
    true
}

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut result = 0;
    for h0 in 0..h {
        for h1 in h0..h {
            for w0 in 0..w {
                for w1 in w0..w {
                    if f(&s, h0, h1, w0, w1) {
                        result += 1;
                    }
                }
            }
        }
    }
    println!("{result}");
}
