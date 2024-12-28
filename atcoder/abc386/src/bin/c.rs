use proconio::{input, marker::Chars};

// 先頭から何文字一致しているかを確認する
fn f(a: &[char], b: &[char]) -> usize {
    let n = a.len().min(b.len());
    for i in 0..n {
        if a[i] != b[i] {
            return i;
        }
    }
    n // すべて一致
}

fn main() {
    input! {
        _k: usize,
        s: Chars,
        t: Chars,
    }

    let i = f(&s, &t);
    let yes = if s == t {
        true
    } else if s.len() == t.len() {
        f(&s[(i + 1)..], &t[(i + 1)..]) == s.len() - (i + 1)
    } else if s.len() + 1 == t.len() {
        f(&s[i..], &t[(i + 1)..]) == s.len() - i
    } else if s.len() == t.len() + 1 {
        f(&s[(i + 1)..], &t[i..]) == t.len() - i
    } else {
        false
    };
    println!("{}", if yes { "Yes" } else { "No" });
}
