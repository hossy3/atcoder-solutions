use proconio::input;

trait PartitionPoint {
    fn partition_point<P>(&self, pred: P) -> isize
    where
        P: Fn(isize) -> bool;
}

impl<T: std::ops::RangeBounds<isize>> PartitionPoint for T {
    /// Returns the index of the partition point according to the given predicate
    /// (the index of the first element of the second partition).
    ///
    /// # Examples
    /// ```
    /// assert_eq!((0..).partition_point(|i| i < 10), 10);
    /// assert_eq!((0..1000).partition_point(|_| false), 0);
    /// assert_eq!((10..=1000).partition_point(|_| true), 1001);
    /// ```
    fn partition_point<P>(&self, pred: P) -> isize
    where
        P: Fn(isize) -> bool,
    {
        let mut r = match self.end_bound() {
            std::ops::Bound::Included(r) => r + 1,
            std::ops::Bound::Excluded(r) => *r,
            std::ops::Bound::Unbounded => isize::MAX,
        };
        let mut l = match self.start_bound() {
            std::ops::Bound::Included(l) => *l,
            std::ops::Bound::Excluded(l) => l + 1,
            std::ops::Bound::Unbounded => isize::MIN,
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
        n: usize,
        k: usize,
        mut a: [isize; n],
    }

    a.sort_unstable();
    // 積がその数「未満」を満たさなくなる最初の組み合わせ数を数える
    let i = (-10isize.pow(18)..=10isize.pow(18)).partition_point(|y| {
        let mut count2 = 0usize; // (i, j), (j, i) を含むので 2倍
        for &x in &a {
            if x > 0 {
                count2 += a.partition_point(|x0| x * x0 < y);
            } else if x < 0 {
                count2 += n - a.partition_point(|x0| x * x0 >= y);
            } else if x == 0 && y > 0 {
                count2 += n;
            }
        }

        // (i, i) を除く。同じ番号は選べない。未満のものが数えられていた
        count2 -= a.iter().filter(|&&x| x * x < y).count();

        let count = count2 / 2;
        // eprintln!("{y}: {count}");
        count < k
    });

    let result = i - 1; // 一つ小さな値が答え
    println!("{result}");
}
