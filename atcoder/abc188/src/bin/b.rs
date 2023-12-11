use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }
    let yes = std::iter::zip(a, b).fold(0, |acc, (a, b)| acc + a * b) == 0;
    println!("{}", if yes { "Yes" } else { "No" });
}
