use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let diff = (0..n).map(|i| a[i].abs_diff(b[i])).sum();
    let yes = (k >= diff) && ((k - diff) % 2 == 0);
    println!("{}", if yes { "Yes" } else { "No" });
}
