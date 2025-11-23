use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let yes = a.windows(2).all(|w| w[0] * a[1] == w[1] * a[0]);
    println!("{}", if yes { "Yes" } else { "No" });
}
