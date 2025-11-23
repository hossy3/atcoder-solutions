use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }
    let sum: usize = a.iter().sum();
    let yes = (0..n).any(|i| sum - a[i] == m);
    println!("{}", if yes { "Yes" } else { "No" });
}
