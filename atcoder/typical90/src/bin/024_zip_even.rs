use num_integer::Integer;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let diff = std::iter::zip(a, b).map(|(a, b)| a.abs_diff(b)).sum();
    let yes = (k >= diff) && (k - diff).is_even();
    println!("{}", if yes { "Yes" } else { "No" });
}
