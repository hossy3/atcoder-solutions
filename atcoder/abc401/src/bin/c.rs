use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    const N: usize = 1_000_000_000_usize;
    let mut v = vec![1usize; n + 1];
    let mut sum = k;
    for i in k..=n {
        v[i] = sum;
        sum = (N + sum * 2 - v[i - k]) % N;
    }

    let result = v[n];
    println!("{result}");
}
