use proconio::input;

const N: usize = 100_000_000;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut cum = vec![0; n + 1];
    for (i, &x) in a.iter().enumerate() {
        cum[i + 1] = cum[i] + x;
    }
    let mut result = 0;
    for (i, &x) in a.iter().enumerate() {
        result += cum[n] - cum[i + 1] + (n - i - 1) * x;
        let j = a.partition_point(|&y| x + y < N);
        if j <= i {
            result -= (n - i - 1) * N;
        } else {
            result -= (n - j) * N;
        }
    }
    println!("{result}");
}
