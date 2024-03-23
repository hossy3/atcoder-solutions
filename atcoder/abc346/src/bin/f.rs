use proconio::{input, marker::Chars};

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

fn f(n: usize, s: &[char], t: &[char], cum: &[[usize; 26]], k: usize) -> bool {
    let mut pos = (0usize, 0usize); // 0 周目, 0 番目
    for &c in t {
        let j = c as usize - 'a' as usize;
        let mut k = k + cum[pos.1][j]; // 文字 c を残り k 個消す
        let k0 = cum[s.len()][j];
        if k0 < k {
            if k0 == 0 {
                return false;
            }
            let m = (k - 1) / k0;
            pos.0 += m;
            k -= m * k0;
            if pos.0 >= n && k > 0 {
                return false;
            }
        }

        let j0 = cum.partition_point(|a| a[j] < k);
        if j0 >= s.len() {
            pos = (pos.0 + 1, 0);
        } else {
            pos.1 = j0;
        }
    }

    pos.0 < n || (pos.0 == n && pos.1 == 0)
}

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }

    let mut cum = vec![[0usize; 26]; s.len() + 1];
    for (i, &c) in s.iter().enumerate() {
        cum[i + 1] = cum[i];
        cum[i + 1][c as usize - 'a' as usize] += 1;
    }

    let result = (0..=(n * s.len() / t.len())).partition_point(|k| f(n, &s, &t, &cum, k)) - 1;
    println!("{result}");
}
