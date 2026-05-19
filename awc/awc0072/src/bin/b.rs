use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [isize; n],
    }

    let mut minus_sum = vec![0; n + 1];
    for (i, &a) in a.iter().enumerate() {
        minus_sum[i + 1] = minus_sum[i] + a.min(0);
    }

    let plus_sum = a.iter().map(|&a| a.max(0)).sum::<isize>();

    let mut result = isize::MIN;
    for l in 0..(n - k + 1) {
        let sum = plus_sum + minus_sum[l + k] - minus_sum[l];
        result = result.max(sum);
    }
    println!("{result}");
}
