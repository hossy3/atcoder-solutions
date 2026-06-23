use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[isize; m]; n],
    }

    let mut sum = vec![vec![0isize; m + 1]; n + 1];
    for i in 0..n {
        for j in 0..m {
            sum[i + 1][j + 1] = sum[i][j + 1] + sum[i + 1][j] - sum[i][j] + a[i][j];
        }
    }

    let mut result = isize::MIN;
    for i0 in 0..n {
        for j0 in 0..m {
            for i1 in (i0 + 1)..=n {
                for j1 in (j0 + 1)..=m {
                    let sum = sum[i1][j1] - sum[i0][j1] - sum[i1][j0] + sum[i0][j0];
                    result = result.max(sum);
                }
            }
        }
    }

    println!("{result}");
}
