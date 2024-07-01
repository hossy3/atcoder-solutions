use proconio::{input, marker::Chars};

fn f(s: &[char], t: &[char]) -> bool {
    for w in 1..(s.len()) {
        for c in 0..w {
            let t0: Vec<_> = s
                .iter()
                .enumerate()
                .filter(|&(i, _)| (i % w) == c)
                .map(|(_, &c)| c)
                .collect();
            if t == t0 {
                return true;
            }
        }
    }
    false
}

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let yes = f(&s, &t);
    println!("{}", if yes { "Yes" } else { "No" });
}
