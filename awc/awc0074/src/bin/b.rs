use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }
    let mut sum = vec![0isize; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + a[i];
    }
    let mut result = 0isize;
    let mut min = 0isize;
    for i in 0..n {
        min = min.min(sum[i]);
        result = result.max(sum[i + 1] - min);
    }
    println!("{result}");
}
