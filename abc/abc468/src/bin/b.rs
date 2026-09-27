use proconio::{input, marker::Chars};

fn main() {
    input! {
        m: usize,
        d: usize,
        s: Chars,
    }

    let mut v = vec![false; m];
    for (i, &c) in s.iter().enumerate() {
        if c != 'G' {
            continue;
        }
        let i0 = i.saturating_sub(d);
        let i1 = (i + d).min(m - 1);
        for i in i0..=i1 {
            v[i] = true;
        }
    }
    let result = v.iter().filter(|&b| !b).count();
    println!("{result}");
}
