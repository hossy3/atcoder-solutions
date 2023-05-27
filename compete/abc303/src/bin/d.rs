use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        s: Chars,
    }
    let n = s.len();
    let mut v = vec![(std::usize::MAX / 2, std::usize::MAX / 2); n + 1]; // [caps off, on]
    v[0].0 = 0;
    for i in 0..n {
        if s[i] == 'a' {
            v[i + 1].0 = (v[i].0 + x).min(v[i].1 + x.min(y) + z);
            v[i + 1].1 = (v[i].0 + x.min(y) + z).min(v[i].1 + y);
        } else {
            v[i + 1].0 = (v[i].0 + y).min(v[i].1 + x.min(y) + z);
            v[i + 1].1 = (v[i].0 + x.min(y) + z).min(v[i].1 + x);
        }
    }
    let result = v[n].0.min(v[n].1);
    println!("{}", result);
}
