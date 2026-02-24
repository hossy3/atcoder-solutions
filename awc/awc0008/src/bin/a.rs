use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        k: usize,
    }
    let yes = (n - 1) * k <= w;
    println!("{}", if yes { "Yes" } else { "No" });
}
