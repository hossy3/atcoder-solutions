use proconio::input;

trait PartitionPoint {
    fn partition_point<P>(&self, pred: P) -> i64
    where
        P: Fn(i64) -> bool;
}

impl<T: std::ops::RangeBounds<i64>> PartitionPoint for T {
    /// Returns the index of the partition point according to the given predicate
    /// (the index of the first element of the second partition).
    ///
    /// # Examples
    /// ```
    /// assert_eq!((0..).partition_point(|i| i < 10), 10);
    /// assert_eq!((0..1000).partition_point(|_| false), 0);
    /// assert_eq!((10..=1000).partition_point(|_| true), 1001);
    /// ```
    fn partition_point<P>(&self, pred: P) -> i64
    where
        P: Fn(i64) -> bool,
    {
        let mut r = match self.end_bound() {
            std::ops::Bound::Included(r) => r + 1,
            std::ops::Bound::Excluded(r) => *r,
            std::ops::Bound::Unbounded => i64::MAX,
        };
        let mut l = match self.start_bound() {
            std::ops::Bound::Included(l) => *l,
            std::ops::Bound::Excluded(l) => l + 1,
            std::ops::Bound::Unbounded => i64::MIN,
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

fn floyd_warshall(mut d: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let n = d.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[i][j] = d[i][j].min(d[i][k] + d[k][j]);
            }
        }
    }
    d
}

fn f(x: i64, p: i64, k: i64, mut a: Vec<Vec<i64>>) -> bool {
    let n = a.len();
    for a in a.iter_mut() {
        for c in a.iter_mut() {
            if *c == -1 {
                *c = x;
            }
        }
    }
    a = floyd_warshall(a);
    let mut count = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            if a[i][j] <= p {
                count += 1;
            }
        }
    }
    count > k
}

fn main() {
    input! {
        (n, p, k): (i64, i64, i64),
        mut a: [[i64; n]; n],
    }

    let r0 = (1..=(p + 1)).partition_point(|i| f(i, p, k, a.clone())) - 1;
    let r1 = (1..=(p + 1)).partition_point(|i| f(i, p, k - 1, a.clone())) - 1;
    let result = r1 - r0;
    if r0 < r1 && r1 == p + 1 {
        println!("Infinity");
    } else {
        println!("{result}");
    }
}
