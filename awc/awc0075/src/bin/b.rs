use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut sum = vec![0usize; n + 1];
    for (i, &a) in a.iter().enumerate() {
        sum[i + 1] = sum[i] + a;
    }

    let mut result = usize::MAX;
    for i in 0..=(n - k) {
        result = result.min(sum[i + k] - sum[i]);
    }
    println!("{result}");
}
