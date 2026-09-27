use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let result = (0..(n - 2))
        .filter(|&i| a[i] < a[i + 1] && a[i + 1] > a[i + 2])
        .count();
    println!("{result}");
}
