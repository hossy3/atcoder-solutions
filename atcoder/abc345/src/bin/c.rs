use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut v = vec![0usize; 26];
    for c in s {
        v[c as usize - 'a' as usize] += 1;
    }
    let mut result = 0;
    for i in 0..26 {
        for j in (i + 1)..26 {
            result += v[i] * v[j];
        }
    }
    if v.iter().any(|&x| x > 1) {
        result += 1;
    }
    println!("{result}");
}
