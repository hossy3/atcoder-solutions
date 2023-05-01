use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        s: Chars,
    }

    let mut v = vec![];
    for (i, &c) in s.iter().enumerate() {
        if c == 'x' {
            v.push(i);
        }
    }

    let mut result = 0;
    for i in 0..v.len() {
        if i > v.len() * m - k {
            break;
        }
        let mut l = if i > 0 { v[i - 1] + 1 } else { 0 };
        let mut r = (((i + k) / v.len()) * n + v[(i + k) % v.len()]).min(n * m);
        if l == 0 {
            if r < n * (m - 1) {
                l = v[v.len() - 1] + 1;
                r += n;
            } else if (n * m - (v[v.len() - 1] + 1)) > r - l {
                l = v[v.len() - 1] + 1;
                r = n * m;
            }
        }
        result = result.max(r - l);
    }
    println!("{}", result);
}
