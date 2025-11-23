use proconio::{input, marker::Chars};

fn f(s: &[char], n: usize) -> i64 {
    let mut x0 = 0usize; // candidate_min
    let mut x1 = 0usize; // candidate_max
    for (i, &c) in s.iter().enumerate() {
        let j = 1 << (s.len() - i - 1);
        match c {
            '0' => {}
            '1' => {
                x0 += j;
                x1 += j;
            }
            _ => {
                if x1 + j <= n {
                    x0 = x1;
                    x1 += j;
                } else if x0 + j + j / 2 <= n {
                    x0 = x0 + j;
                    x1 = x1.max(x0);
                } else if x0 + j <= n {
                    x1 = x0 + j;
                }
            }
        };
        if x0 > n {
            return -1;
        }
    }

    let result = if x1 <= n { x1 } else { x0 };
    result as i64
}

#[test]
fn test_func_1() {
    let v = ['?', '0', '?'];
    assert_eq!(f(&v, 0), 0);
    assert_eq!(f(&v, 1), 1);
    assert_eq!(f(&v, 2), 1);
    assert_eq!(f(&v, 3), 1);
    assert_eq!(f(&v, 4), 4);
    assert_eq!(f(&v, 5), 5);
    assert_eq!(f(&v, 6), 5);
    assert_eq!(f(&v, 7), 5);
    assert_eq!(f(&v, 8), 5);
}

#[test]
fn test_func_2() {
    let v = ['?', '1', '?'];
    assert_eq!(f(&v, 0), -1);
    assert_eq!(f(&v, 1), -1);
    assert_eq!(f(&v, 2), 2);
    assert_eq!(f(&v, 3), 3);
    assert_eq!(f(&v, 4), 3);
    assert_eq!(f(&v, 5), 3);
    assert_eq!(f(&v, 6), 6);
    assert_eq!(f(&v, 7), 7);
    assert_eq!(f(&v, 8), 7);
}

fn main() {
    input! {
        s: Chars,
        n: usize,
    }
    let result = f(&s, n);
    println!("{}", result);
}
