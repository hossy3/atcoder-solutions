use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn f(s: &[char], t: usize, k: usize, offset: usize) -> char {
    if k == 0 || t == 0 {
        let i = ((s[k] as usize - 'A' as usize) + t + offset) % 3;
        ['A', 'B', 'C'][i]
    } else {
        f(s, t - 1, k / 2, (offset + k % 2 + 1) % 3)
    }
}

#[test]
fn test_func() {
    use itertools::Itertools;

    assert_eq!(f(&"ABC".chars().collect_vec(), 0, 0, 0), 'A');
    assert_eq!(f(&"ABC".chars().collect_vec(), 0, 1, 0), 'B');
    assert_eq!(f(&"ABC".chars().collect_vec(), 0, 2, 0), 'C');

    assert_eq!(f(&"ABC".chars().collect_vec(), 1, 0, 0), 'B');
    assert_eq!(f(&"ABC".chars().collect_vec(), 1, 1, 0), 'C');
    assert_eq!(f(&"ABC".chars().collect_vec(), 1, 2, 0), 'C');
    assert_eq!(f(&"ABC".chars().collect_vec(), 1, 3, 0), 'A');
    assert_eq!(f(&"ABC".chars().collect_vec(), 1, 4, 0), 'A');
    assert_eq!(f(&"ABC".chars().collect_vec(), 1, 5, 0), 'B');
}

fn main() {
    input! {
        s: Chars,
        q: usize,
        tk: [(usize, Usize1); q],
    }

    for &(t, k) in &tk {
        let result = f(&s, t, k, 0);
        println!("{}", result);
    }
}
