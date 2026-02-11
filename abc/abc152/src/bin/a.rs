use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let yes = n == m;
    println!("{}", if yes { "Yes" } else { "No" });
}
