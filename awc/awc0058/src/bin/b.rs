use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        h: [usize; n],
    }

    let mut counts = vec![0usize; n];
    for (i, &h) in h.iter().enumerate() {
        if h >= k {
            counts[i] += h;
        }
    }

    let mut cum = vec![0usize; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + counts[i];
    }

    let result = (0..n).map(|i| cum[(i + m).min(n)] - cum[i]).max().unwrap();
    println!("{result}");
}
