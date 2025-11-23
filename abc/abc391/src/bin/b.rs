use proconio::{input, marker::Chars};

fn g(s: &[Vec<char>], t: &[Vec<char>], i: usize, j: usize) -> bool {
    let m = t.len();
    for k in 0..m {
        for l in 0..m {
            if s[i + k][j + l] != t[k][l] {
                return false;
            }
        }
    }
    true
}

fn f(s: &[Vec<char>], t: &[Vec<char>]) -> (usize, usize) {
    let n = s.len();
    let m = t.len();
    for i in 0..=(n - m) {
        for j in 0..=(n - m) {
            if g(s, t, i, j) {
                return (i + 1, j + 1);
            }
        }
    }
    unreachable!();
}
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
        t: [Chars; m],
    }
    let (a, b) = f(&s, &t);
    println!("{a} {b}");
}
