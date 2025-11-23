use proconio::input;

// bin-search

fn f(w: usize, m: usize, l: &[usize]) -> bool {
    let mut lines = 1usize;
    let mut c = 0usize;
    for &x in l {
        if x > w {
            return false;
        }
        if c == 0 {
            c = x;
        } else if c + 1 + x > w {
            c = x;
            lines += 1;
            if lines > m {
                return false;
            }
        } else {
            c += 1 + x;
        }
    }
    true
}

#[test]
fn test_func() {
    let l = vec![9, 5, 2, 7, 1, 8, 8, 2, 1, 5, 2, 3, 6];
    assert_eq!(f(25, 3, &l), false);
    assert_eq!(f(26, 3, &l), true);
    assert_eq!(f(27, 3, &l), true);

    assert_eq!(min_left(0, 10000, |i| i > 5000), 5001);
    assert_eq!(max_right(0, 10000, |i| i < 5000), 4999);
}

/// # Examples
/// ```
/// assert_eq!(min_left(0, 10000, |i| i > 5000), 5001);
/// ```
fn max_right<F>(mut l: i64, mut r: i64, f: F) -> i64
where
    F: Fn(i64) -> bool,
{
    while l != r {
        let w = (l + r) / 2;
        if f(w) {
            l = w + 1;
        } else {
            r = w;
        }
    }
    l - 1
}

/// # Examples
/// ```
/// assert_eq!(min_left(0, 10000, |i| i > 5000), 5001);
/// ```
fn min_left<F>(mut l: i64, mut r: i64, f: F) -> i64
where
    F: Fn(i64) -> bool,
{
    while l != r {
        let w = (l + r) / 2;
        if f(w) {
            r = w;
        } else {
            l = w + 1;
        }
    }
    l
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ls: [usize; n],
    }
    let result = min_left(1, 200_001 * 1_000_000_000, |w| f(w as usize, m, &ls));
    println!("{result}");
}
