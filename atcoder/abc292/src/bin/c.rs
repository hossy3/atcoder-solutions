use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut v = vec![0usize; n];
    for i in 1..n {
        let jmax = (n - 1) / i;
        for j in 1..=jmax {
            v[i * j] += 1;
        }
    }
    let mut result = 0;
    for i in 1..n {
        result += v[i] * v[n - i];
    }
    println!("{}", result);
}
