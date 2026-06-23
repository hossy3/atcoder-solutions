use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[isize; m]; n],
    }

    let mut sum = vec![vec![0isize; m]; n + 1]; // 縦方向のみ累積和
    for i in 0..n {
        for j in 0..m {
            sum[i + 1][j] = sum[i][j] + a[i][j];
        }
    }

    let mut result = isize::MIN;
    for i0 in 0..n {
        for i1 in (i0 + 1)..=n {
            let mut min = 0;
            let mut cur = 0;
            for j in 0..m {
                cur += sum[i1][j] - sum[i0][j];
                result = result.max(cur - min);
                min = min.min(cur);
            }
        }
    }
    println!("{result}");
}
