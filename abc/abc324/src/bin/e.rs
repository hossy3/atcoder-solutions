use proconio::{input, marker::Chars};

fn f(s: &[char], t: &[char]) -> usize {
    let mut s0 = 0;
    let mut t0 = 0;
    while let Some(i) = (s0..s.len()).position(|i| s[i] == t[t0]) {
        s0 = s0 + i + 1;
        t0 += 1;
        if t0 == t.len() {
            break;
        }
    }
    t0
}

fn g(s: &[char], t: &[char]) -> usize {
    let mut s0 = 0;
    let mut t0 = 0;
    while let Some(i) = (s0..s.len()).position(|i| s[s.len() - i - 1] == t[t.len() - t0 - 1]) {
        s0 = s0 + i + 1;
        t0 += 1;
        if t0 == t.len() {
            break;
        }
    }
    t0
}

#[test]
fn test_func() {
    use itertools::Itertools;
    assert_eq!(
        f(&"xxxx".chars().collect_vec(), &"xxxxxxxx".chars().collect_vec()),
        4
    );
    assert_eq!(
        g(&"xxxx".chars().collect_vec(), &"xxxxxxxx".chars().collect_vec()),
        4
    );

    assert_eq!(
        g(&"da".chars().collect_vec(), &"abc".chars().collect_vec()),
        0
    );
    assert_eq!(
        f(&"da".chars().collect_vec(), &"abc".chars().collect_vec()),
        1
    );

    assert_eq!(
        g(&"abc".chars().collect_vec(), &"abc".chars().collect_vec()),
        3
    );
    assert_eq!(
        g(&"abc".chars().collect_vec(), &"ab".chars().collect_vec()),
        2
    );
    assert_eq!(
        g(&"abc".chars().collect_vec(), &"a".chars().collect_vec()),
        1
    );
    assert_eq!(
        g(&"abc".chars().collect_vec(), &"ba".chars().collect_vec()),
        1
    );
    assert_eq!(
        f(&"abc".chars().collect_vec(), &"ba".chars().collect_vec()),
        1
    );
    assert_eq!(
        g(&"abc".chars().collect_vec(), &"da".chars().collect_vec()),
        1
    );
    assert_eq!(
        f(&"abc".chars().collect_vec(), &"da".chars().collect_vec()),
        0
    );

}

fn main() {
    input! {
        n: usize,
        t: Chars,
        s: [Chars; n],
    }
    let mut v = Vec::with_capacity(n);
    for s in s {
        v.push((f(&s, &t), g(&s, &t)));
    }

    let mut v0 = vec![0usize; t.len() + 1];
    let mut v1 = vec![0usize; t.len() + 1];
    for (i, j) in v {
        v0[i] += 1;
        v1[j] += 1;
    }
    for i in (0..(t.len())).rev() {
        v1[i] += v1[i + 1];
    }

    let mut result = 0;
    for i in 0..=(t.len()) {
        result += v0[i] * v1[t.len() - i];
    }
    println!("{result}");
}
