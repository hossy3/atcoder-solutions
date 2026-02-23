use proconio::input;

/// !pred(x) となる最も小さな値を返す。 [..., true, false, ...] のように並ぶこと
fn partition_point_range<P>(range: std::ops::Range<usize>, pred: P) -> usize
where
    P: Fn(usize) -> bool,
{
    let mut l = range.start;
    let mut r = range.end;
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
    let result = partition_point_range(1..(sum + 1), |i| {
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
