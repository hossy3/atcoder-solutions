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
        a: [usize; n],
    }

    let min = *a.iter().max().unwrap();
    let max = a.iter().sum::<usize>();
    let result = partition_point(min..=max, |x| {
        let mut count = 1usize;
        let mut cur = 0usize;
        for &a in &a {
            if cur + a > x {
                count += 1;
                cur = a;
            } else {
                cur += a;
            }
        }
        // eprintln!("{count} {k} {x}");
        count > k + 1
    });
    println!("{result}");
}
