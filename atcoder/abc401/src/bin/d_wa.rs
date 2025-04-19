use itertools::Itertools;
use proconio::{input, marker::Chars};

fn f(n: usize, k: usize, mut s: Vec<char>) -> Vec<char> {
    if k == 0 {
        return vec!['.'; n];
    }

    // 確実に "." になる場所を埋める
    for i in 0..(n - 1) {
        if s[i] == 'o' && s[i + 1] == '?' {
            s[i + 1] = '.';
        } else if s[i] == '?' && s[i + 1] == 'o' {
            s[i] = '.';
        }
    }

    let mut s0 = s.clone();
    if s0[0] == '?' {
        s0[0] = 'o';
    }
    for i in 0..(n - 1) {
        if s0[i + 1] == '?' {
            s0[i + 1] = if s0[i] == 'o' { '.' } else { 'o' };
        }
    }

    let mut s1 = s.clone();
    s1.reverse();
    if s1[0] == '?' {
        s1[0] = 'o';
    }
    for i in 0..(n - 1) {
        if s1[i + 1] == '?' {
            s1[i + 1] = if s1[i] == 'o' { '.' } else { 'o' };
        }
    }
    s1.reverse();

    let mut s2 = s1.clone();
    for i in 0..n {
        if s0[i] != s1[i] {
            s2[i] = '?';
        }
    }

    if s0.iter().filter(|&&c| c == 'o').count() == k
        && s1.iter().filter(|&&c| c == 'o').count() == k
    {
        s2
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(
            f(5, 2, "o????".chars().collect_vec()),
            "o.???".chars().collect_vec()
        );
        assert_eq!(
            f(6, 3, "?.??.?".chars().collect_vec()),
            "o.??.o".chars().collect_vec()
        );
        assert_eq!(
            f(7, 3, "??.??.?".chars().collect_vec()),
            "??.??.o".chars().collect_vec()
        );
        assert_eq!(
            f(9, 4, "??.??.???".chars().collect_vec()),
            "??.??.o.o".chars().collect_vec()
        );
        assert_eq!(
            f(9, 4, "o?.??.o??".chars().collect_vec()),
            "o..??.o.o".chars().collect_vec()
        );
        assert_eq!(
            f(9, 4, "?o.??.??o".chars().collect_vec()),
            ".o.??.o.o".chars().collect_vec()
        );
        assert_eq!(
            f(9, 4, "??.o?.???".chars().collect_vec()),
            "??.o..o.o".chars().collect_vec()
        );
        assert_eq!(
            f(7, 3, "?o?????".chars().collect_vec()),
            ".o.????".chars().collect_vec()
        );
        assert_eq!(
            f(7, 3, "o??????".chars().collect_vec()),
            "o.?????".chars().collect_vec()
        );
        assert_eq!(
            f(7, 3, "??o????".chars().collect_vec()),
            "?.o.???".chars().collect_vec()
        );
        assert_eq!(
            f(7, 3, "??o???o".chars().collect_vec()),
            "?.o.?.o".chars().collect_vec()
        );
        assert_eq!(
            f(7, 3, "??o??o?".chars().collect_vec()),
            "o.o..o.".chars().collect_vec()
        );
        assert_eq!(
            f(2, 0, "??".chars().collect_vec()),
            "..".chars().collect_vec()
        );
        assert_eq!(
            f(3, 1, "??o".chars().collect_vec()),
            "...".chars().collect_vec()
        );
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let result = f(n, k, s);
    println!("{}", result.iter().join(""));
}
