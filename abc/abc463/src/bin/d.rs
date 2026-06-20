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
        mut lr: [(usize, usize); n],
    }

    // 布の右端で並び替えていちばん詰める
    lr.sort_unstable_by_key(|&(_, r)| r);

    let result = partition_point(1..=1_000_000_000, |w| {
        let mut cur = 0usize;
        let mut v = vec![];
        for &(l, r) in &lr {
            if cur <= l {
                v.push(r);
                if v.len() == k {
                    return true;
                }
                cur = r + w;
            }
        }
        false
    }) - 1;

    if result == 0 {
        println!("-1");
    } else {
        println!("{result}");
    }
}
