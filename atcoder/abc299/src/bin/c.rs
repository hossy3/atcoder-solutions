use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        mut s: Chars,
    }
    if s.iter().all(|&x| x == 'o') || s.iter().all(|&x| x == '-') {
        println!("{}", -1);
        return;
    }

    s.push('-');
    let mut score = 0;
    let mut cur = 0;
    for &c in &s {
        if c == 'o' {
            cur += 1;
        } else {
            score = score.max(cur);
            cur = 0;
        }
    }
    println!("{}", score);
}
