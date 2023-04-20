use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        t: Chars,
    }
    let a = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut dir = 0;
    let mut p = (0, 0);
    for c in t {
        if c == 'S' {
            p.0 += a[dir].0;
            p.1 += a[dir].1;
        } else {
            dir = (dir + 1) % 4;
        }
    }
    println!("{} {}", p.0, p.1);
}
