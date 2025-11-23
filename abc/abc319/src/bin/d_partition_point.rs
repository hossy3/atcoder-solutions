use proconio::input;

// bin-search

fn f(w: usize, m: usize, l: &[usize]) -> bool {
    let mut lines = 1usize;
    let mut c = 0usize;
    for &x in l {
        if x > w {
            return true;
        }
        if c == 0 {
            c = x;
        } else if c + 1 + x > w {
            c = x;
            lines += 1;
            if lines > m {
                return true;
            }
        } else {
            c += 1 + x;
        }
    }
    false
}

#[test]
fn test_func() {
    let l = vec![9, 5, 2, 7, 1, 8, 8, 2, 1, 5, 2, 3, 6];
    assert_eq!(f(25, 3, &l), true);
    assert_eq!(f(26, 3, &l), false);
    assert_eq!(f(27, 3, &l), false);

    assert_eq!(partition_point(0.., |i| i < 10), 10);
    assert_eq!(partition_point(0..1000, |_| false), 0);
    assert_eq!(partition_point(10..=1000, |_| true), 1001);
}

/// Returns the index of the partition point according to the given predicate
/// (the index of the first element of the second partition).
///
/// # Examples
/// ```
/// assert_eq!(partition_point(0.., |i| i < 10), 10);
/// assert_eq!(partition_point(0..1000, |_| false), 0);
/// assert_eq!(partition_point(10..=1000, |_| true), 1001);
/// ```
pub fn partition_point<R, P>(range: R, pred: P) -> usize
where
    R: std::ops::RangeBounds<usize>,
    P: Fn(usize) -> bool,
{
    const R_MAX: usize = usize::MAX / 2 + 1;
    let mut r = match range.end_bound() {
        std::ops::Bound::Included(r) => r + 1,
        std::ops::Bound::Excluded(r) => *r,
        std::ops::Bound::Unbounded => R_MAX,
    };
    let mut l = match range.start_bound() {
        std::ops::Bound::Included(l) => *l,
        std::ops::Bound::Excluded(l) => l + 1,
        std::ops::Bound::Unbounded => 0,
    };

    assert!(l <= r && r <= R_MAX);
    while l != r {
        let m = (l + r) / 2;
        if pred(m) {
            l = m + 1;
        } else {
            r = m;
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
    let result = partition_point(1..(200_001 * 1_000_000_000), |w| f(w as usize, m, &ls));
    println!("{result}");
}
