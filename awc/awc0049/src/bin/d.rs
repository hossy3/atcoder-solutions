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
        m: usize,
        k: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }

    a.sort_unstable();
    b.sort_unstable();

    a.reverse();
    b.reverse();

    let min = a.iter().min().unwrap() * b.iter().min().unwrap();
    let max = a.iter().max().unwrap() * b.iter().max().unwrap();

    // 先頭から k 番目以上になるもっとも大きな数 = k 番目未満のもっとも大きな数 + 1
    let k1 = partition_point(min..=max, |x| {
        let mut count = 0;
        for &a in &a {
            count += b.partition_point(|&b| a * b >= x);
        }
        count >= k
    });

    // 累積和
    let mut b_cum = vec![0; m + 1];
    for i in 0..m {
        b_cum[i + 1] = b_cum[i] + b[i];
    }

    // 先頭から k 未満の個数と合計数
    let mut n0 = 0usize;
    let mut sum0 = 0usize;
    for &a in &a {
        let n = b.partition_point(|&b| a * b >= k1);
        n0 += n;
        sum0 += a * b_cum[n];
    }

    let result = sum0 + (k1 - 1) * (k - n0);
    println!("{result}");
}
