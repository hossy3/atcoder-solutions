use itertools::Itertools;
use proconio::input;

fn combination(n: usize, r: usize) -> usize {
    let r = r.min(n - r);
    (1..=r).fold(1, |acc, x| acc * (n - x + 1) / x)
}

fn f(mut a: usize, mut b: usize, mut k: usize) -> Vec<char> {
    let mut s = vec![];
    while a + b > 0 {
        if a == 0 {
            b -= 1;
            s.push('b');
        } else if k > combination(a + b - 1, a - 1) {
            k -= combination(a + b - 1, a - 1);
            b -= 1;
            s.push('b');
        } else {
            a -= 1;
            s.push('a');
        }
    }
    s
}

#[test]
fn test_func() {
    assert_eq!(f(1, 1, 1), "ab".chars().collect_vec());
    assert_eq!(f(1, 1, 2), "ba".chars().collect_vec());

    assert_eq!(f(2, 1, 1), "aab".chars().collect_vec());
    assert_eq!(f(2, 1, 2), "aba".chars().collect_vec());
    assert_eq!(f(2, 1, 3), "baa".chars().collect_vec());
    assert_eq!(f(1, 2, 1), "abb".chars().collect_vec());
    assert_eq!(f(1, 2, 2), "bab".chars().collect_vec());
    assert_eq!(f(1, 2, 3), "bba".chars().collect_vec());
}

fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
    }
    let s = f(a, b, k);
    println!("{}", s.iter().join(""));
}
