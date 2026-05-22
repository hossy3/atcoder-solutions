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
        std::ops::Bound::Unbounded => usize::MIN,
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
        m: usize,
        mut a: [usize; n],
    }

    if a.len() == 1 {
        println!("0");
        return;
    }

    a.sort();

    let max = a[a.len() - 1];
    let result = partition_point(0..=max, |d| {
        let mut l = 0;
        let mut count = 1;
        let mut min = a[0];
        let mut max = a[0];
        for r in 1..a.len() {
            min = min.min(a[r]);
            max = max.max(a[r]);
            if r - l >= m || max - min > d {
                l = r;
                min = a[r];
                max = a[r];
                count += 1;
            }
        }
        count > k
    });
    println!("{result}");
}
