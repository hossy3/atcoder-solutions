use proconio::{input, marker::Chars};

fn f(s: &[char], t: &[char]) -> isize {
    let mut result = 0isize;

    let mut j = 0usize;
    for &s0 in s {
        if s0 == 'A' {
            if j == t.len() || t[j] != 'A' {
                result += 1; // この A を削除する
            } else {
                j += 1;
            }
        } else {
            while j < t.len() && t[j] != s0 {
                if t[j] != 'A' {
                    return -1;
                }
                result += 1; // この A を削除する
                j += 1;
            }
            if j == t.len() {
                return -1;
            }
            j += 1;
        }
    }

    if t[j..].iter().filter(|&&x| x != 'A').count() > 0 {
        return -1;
    }
    result += (t.len() - j) as isize;
    result
}

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let result = f(&s, &t);
    println!("{result}");
}
