use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let diff = std::iter::zip(a, b).map(|(a, b)| a.abs_diff(b)).sum();
    let yes = (k >= diff) && ((k - diff) % 2 == 0);
    println!("{}", if yes { "Yes" } else { "No" });
}
