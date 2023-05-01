use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let yes = n == 1 || n > 4;
    println!("{}", if yes { "Yes" } else { "No" });
}
