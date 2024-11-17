use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize; m],
        a: [usize; m],
    }

    let mut cum = vec![0usize; n + 1];
    for i in 0..m {
        cum[x[i]] = a[i];
    }
    for i in 0..n {
        cum[i + 1] += cum[i];
    }

    if cum[n] != n || (1..=n).any(|i| cum[i] < i) {
        println!("-1");
        return;
    }

    let mut cum = vec![0usize; n + 1];
    for i in 0..m {
        cum[x[i]] = a[i] * x[i];
    }
    for i in 0..n {
        cum[i + 1] += cum[i];
    }
    let result = (n * (n + 1)) / 2 - cum[n];
    println!("{result}");
}
