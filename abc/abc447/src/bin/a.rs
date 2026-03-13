use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let yes = (n + 1) >= m * 2;
    println!("{}", if yes { "Yes" } else { "No" });
}
