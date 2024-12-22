use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut v = vec![0usize; 26];
    for &x in &s {
        let i = x as usize - 'a' as usize;
        v[i] += 1;
    }
    let yes =
        (0..(s.len() / 2)).all(|i| s[i * 2] == s[i * 2 + 1]) && v.iter().all(|&x| x == 0 || x == 2);
    println!("{}", if yes { "Yes" } else { "No" });
}
