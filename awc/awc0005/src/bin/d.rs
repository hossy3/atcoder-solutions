use proconio::input;

/// !pred(x) となる最も小さな値を返す。 [..., true, false, ...] のように並ぶこと
///
/// ```
/// assert_eq!(partition_point(0.., |i| i < 10), 10);
/// assert_eq!(partition_point(0..1000, |_| false), 0);
/// assert_eq!(partition_point(10..=1000, |_| true), 1001);
/// ```
fn partition_point<R, P>(range: R, pred: P) -> usize
where
    R: std::ops::RangeBounds<usize>,
    P: Fn(usize) -> bool,
{
    let mut r = match range.end_bound() {
        std::ops::Bound::Included(r) => r + 1,
        std::ops::Bound::Excluded(r) => *r,
        std::ops::Bound::Unbounded => usize::MAX,
    };
    let mut l = match range.start_bound() {
        std::ops::Bound::Included(l) => *l,
        std::ops::Bound::Excluded(l) => l + 1,
        std::ops::Bound::Unbounded => 0,
    };
    while l != r {
        let mid = l + (r - l) / 2;
        if pred(mid) {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    l
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let sum: usize = a.iter().sum();
    // 条件を満たせば false を返す
    let result = partition_point(1..(sum + 1), |i| {
        let mut count = 0;
        let mut cur = 0;
        for &x in &a {
            cur += x;
            if cur >= i {
                count += 1;
                cur = 0;
            }
        }
        count >= k
    }) - 1;

    println!("{result}");
}
