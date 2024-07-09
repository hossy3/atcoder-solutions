use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }
    a.sort();
    let result = (0..=k).map(|i| a[i + n - k - 1] - a[i]).min().unwrap();
    println!("{result}");
}
