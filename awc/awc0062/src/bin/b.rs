use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        g: usize,
        dt: [(usize, usize); n],
    }

    let mut cum = vec![0usize; n + 1];
    for (i, &(_, t)) in dt.iter().enumerate() {
        cum[i + 1] = cum[i] + t;
    }
    let max = (0..=(n - k)).map(|i| cum[i + k] - cum[i]).max().unwrap();
    let result = g + cum[n] - max;
    println!("{result}");
}
