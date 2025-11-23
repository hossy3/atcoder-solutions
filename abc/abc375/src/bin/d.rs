use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut m = vec![vec![]; 26];
    for (i, &c) in s.iter().enumerate() {
        let j = c as usize - 'A' as usize;
        m[j].push(i);
    }

    let mut result = 0;
    for v in m {
        if v.len() < 2 {
            continue;
        }
        let n = v.len();
        let mut cum = vec![0; n + 1];
        for (i, &x) in v.iter().enumerate() {
            cum[i + 1] = cum[i] + x;
        }
        for i in 0..(n - 1) {
            result += cum[n] - cum[i + 1] - (n - i - 1) * (v[i] + 1);
        }
    }
    println!("{result}");
}
