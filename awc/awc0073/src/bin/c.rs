use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        mut xs: [(usize, usize); n],
    }

    xs.sort_unstable();

    let mut sum = vec![0usize; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + xs[i].1;
    }

    // eprintln!("{:?}", &sum);

    let mut result = 0;
    for i in 0..(n - 1) {
        let j = xs.partition_point(|&(x, _)| xs[i].0 + d >= x);
        // eprintln!("{}, {}, {}, {}", i, j, sum[j], sum[i + 1]);
        result += xs[i].1 * (sum[j] - sum[i + 1]);
    }
    println!("{result}");
}
