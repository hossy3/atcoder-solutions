use num_integer::Integer;
use proconio::input;

trait PartitionPoint {
    fn partition_point<P>(&self, pred: P) -> usize
    where
        P: Fn(usize) -> bool;
}

impl<T: std::ops::RangeBounds<usize>> PartitionPoint for T {
    /// Returns the index of the partition point according to the given predicate
    /// (the index of the first element of the second partition).
    ///
    /// # Examples
    /// ```
    /// assert_eq!((0..).partition_point(|i| i < 10), 10);
    /// assert_eq!((0..1000).partition_point(|_| false), 0);
    /// assert_eq!((10..=1000).partition_point(|_| true), 1001);
    /// ```
    fn partition_point<P>(&self, pred: P) -> usize
    where
        P: Fn(usize) -> bool,
    {
        let mut r = match self.end_bound() {
            std::ops::Bound::Included(r) => r + 1,
            std::ops::Bound::Excluded(r) => *r,
            std::ops::Bound::Unbounded => usize::MAX,
        };
        let mut l = match self.start_bound() {
            std::ops::Bound::Included(l) => *l,
            std::ops::Bound::Excluded(l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };

        assert!(l <= r);
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
}

fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
    }
    let lcm = n.lcm(&m);
    let result = (1..=(usize::MAX / 2)).partition_point(|i| {
        let k0 = i / n + i / m - i / lcm * 2;
        k0 < k
    });
    println!("{result}");
}
