use proconio::input;

/// !pred(x) となる最も小さな値を返す。 [..., true, false, ...] のように並ぶこと
fn partition_point_range<P>(range: std::ops::Range<isize>, pred: P) -> isize
where
    P: Fn(isize) -> bool,
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
        mut a: [isize; n],
    }

    a.sort_unstable();
    // 積がその数「未満」を満たさなくなる最初の組み合わせ数を数える
    let i = partition_point_range(-10isize.pow(18)..(10isize.pow(18) + 1), |y| {
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
