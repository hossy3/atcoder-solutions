use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [usize; n],
    }

    let mut sum = vec![0usize; n + 1]; // 累積和
    for (i, &l) in l.iter().enumerate() {
        sum[i + 1] = sum[i] + l;
    }

    let mut result = usize::MAX;
    for i in 0..=n {
        result = result.min((sum[n] - sum[i]).abs_diff(sum[i]));
    }
    println!("{result}");
}
