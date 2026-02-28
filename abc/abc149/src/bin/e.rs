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
        m: usize,
        mut a: [usize; n],
    }
    a.sort();
    a.reverse();

    // 合計数の中で満たす最小値を二分探索で求める
    let min = a[a.len() - 1] * 2 + 1;
    let max = a[0] * 2;
    let sum_min = partition_point_range(min..max, |x| {
        let mut count = 0;
        for &y in &a {
            count += a.partition_point(|&z| y + z >= x);
        }
        count >= m
    }) - 1;

    // 累積和を使って、最小値以上になる組み合わせの合計をとる
    let mut cum = vec![0; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + a[i];
    }

    let mut result = 0;
    let mut count = 0;
    for &y in &a {
        let i = a.partition_point(|&z| y + z >= sum_min);
        result += y * i + cum[i];
        count += i;
    }
    result -= (count - m) * sum_min; // 足しすぎている場合は除く

    println!("{result}");
}
