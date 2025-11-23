use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let yes = a.windows(2).all(|w| w[0] < w[1]);
    println!("{}", if yes { "Yes" } else { "No" });
}
