use proconio::{input, marker::Chars};

fn f(s: Vec<char>) -> char {
    let n = s.len();
    let mut s0 = Vec::with_capacity(n - 1); // A
    s0.push(s[0]);
    for i in 0..(n - 1) {
        if s[i] != s[i + 1] {
            s0.push(s[i + 1]);
        }
    }
    if s0[0] == 'A' {
        if s0.len() == 2 { 'B' } else { 'A' }
    } else {
        if s0.len() == 1 { 'B' } else { 'A' }
    }
}

#[test]
fn test_func() {
    use itertools::Itertools;
    assert_eq!(f("ABBABA".chars().collect_vec()), 'A');
    assert_eq!(f("ABBBABA".chars().collect_vec()), 'A');
    assert_eq!(f("ABBBBABA".chars().collect_vec()), 'A');
    assert_eq!(f("ABABA".chars().collect_vec()), 'A');
    assert_eq!(f("ABABA".chars().collect_vec()), 'A');
    assert_eq!(f("ABAABA".chars().collect_vec()), 'A');
    assert_eq!(f("ABAAABA".chars().collect_vec()), 'A');
    assert_eq!(f("ABAAAABA".chars().collect_vec()), 'A');

    assert_eq!(f("ABA".chars().collect_vec()), 'A');
    assert_eq!(f("ABAA".chars().collect_vec()), 'A');
    assert_eq!(f("ABAAA".chars().collect_vec()), 'A');
    assert_eq!(f("ABAAAA".chars().collect_vec()), 'A');

    assert_eq!(f("AB".chars().collect_vec()), 'B');
    assert_eq!(f("AAA".chars().collect_vec()), 'A');
    assert_eq!(f("ABAB".chars().collect_vec()), 'A');
}

#[test]
fn test_func1() {
    use itertools::Itertools;
    assert_eq!(f("A".chars().collect_vec()), 'A');
    assert_eq!(f("B".chars().collect_vec()), 'B');
}

#[test]
fn test_func2() {
    use itertools::Itertools;
    assert_eq!(f("AA".chars().collect_vec()), 'A');
    assert_eq!(f("AB".chars().collect_vec()), 'B');
    assert_eq!(f("BA".chars().collect_vec()), 'A');
    assert_eq!(f("BB".chars().collect_vec()), 'B');
}

#[test]
fn test_func3() {
    use itertools::Itertools;
    assert_eq!(f("AAA".chars().collect_vec()), 'A');
    assert_eq!(f("AAB".chars().collect_vec()), 'B');
    assert_eq!(f("ABA".chars().collect_vec()), 'A');
    assert_eq!(f("ABB".chars().collect_vec()), 'B');
    assert_eq!(f("BAA".chars().collect_vec()), 'A');
    assert_eq!(f("BAB".chars().collect_vec()), 'A');
    assert_eq!(f("BBA".chars().collect_vec()), 'A');
    assert_eq!(f("BBB".chars().collect_vec()), 'B');
}

#[test]
fn test_func4() {
    use itertools::Itertools;
    assert_eq!(f("AAAA".chars().collect_vec()), 'A');
    assert_eq!(f("AAAB".chars().collect_vec()), 'B');
    assert_eq!(f("AABA".chars().collect_vec()), 'A');
    assert_eq!(f("AABB".chars().collect_vec()), 'B');
    assert_eq!(f("ABAA".chars().collect_vec()), 'A');
    assert_eq!(f("ABAB".chars().collect_vec()), 'A');
    assert_eq!(f("ABBA".chars().collect_vec()), 'A');
    assert_eq!(f("ABBB".chars().collect_vec()), 'B');
    assert_eq!(f("BAAA".chars().collect_vec()), 'A');
    assert_eq!(f("BAAB".chars().collect_vec()), 'A');
    assert_eq!(f("BABA".chars().collect_vec()), 'A');
    assert_eq!(f("BABB".chars().collect_vec()), 'A');
    assert_eq!(f("BBAA".chars().collect_vec()), 'A');
    assert_eq!(f("BBAB".chars().collect_vec()), 'A');
    assert_eq!(f("BBBA".chars().collect_vec()), 'A');
    assert_eq!(f("BBBB".chars().collect_vec()), 'B');
}


fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            _: usize,
            s: Chars,
        }
        let result = f(s);
        println!("{}", result);
    }
}
