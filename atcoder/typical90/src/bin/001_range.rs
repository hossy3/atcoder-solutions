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

fn f(l: usize, k: usize, v: &[usize]) -> bool {
    let mut num = 0;
    let mut cur = 0;
    for &x in v {
        cur += x;
        if cur >= l {
            num += 1;
            cur = 0;
        }
    }
    num > k
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func() {
        assert_eq!(f(12, 2, &[8, 5, 13, 8]), true);
        assert_eq!(f(13, 2, &[8, 5, 13, 8]), true);
        assert_eq!(f(14, 2, &[8, 5, 13, 8]), false);
    }
}

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        a: [usize; n],
    }
    let mut v = Vec::with_capacity(n + 1);
    v.push(a[0]);
    for x in a.windows(2) {
        v.push(x[1] - x[0]);
    }
    v.push(l - a[n - 1]);

    let result = (1..=l).partition_point(|i| f(i, k, &v)) - 1;
    println!("{result}");
}
