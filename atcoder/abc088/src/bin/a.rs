use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
    }
    let yes = n % 500 <= a;
    println!("{}", if yes { "Yes" } else { "No" });
}
