use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut cum = a.clone();
    for i in 0..(n - 1) {
        cum[i + 1] += cum[i];
    }
    let min = *cum.iter().min().unwrap_or(&0);
    let result = cum[n - 1] - min.min(0);
    println!("{result}");
}
