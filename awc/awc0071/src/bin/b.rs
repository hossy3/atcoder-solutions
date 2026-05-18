use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut v: Vec<(char, usize)> = vec![];
    for &c in &s {
        if v.len() == 0 || v[v.len() - 1].0 != c {
            v.push((c, 1));
        } else {
            let i = v.len() - 1;
            v[i].1 += 1;
        }
    }

    let mut result = 0usize;
    for v in v.windows(3) {
        if v[0].0 == v[2].0 {
            result += v[0].1 * v[2].1;
        }
    }
    println!("{result}");
}
