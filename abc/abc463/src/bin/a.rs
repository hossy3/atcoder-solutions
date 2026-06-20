use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }
    let yes = x * 9 == y * 16;
    println!("{}", if yes { "Yes" } else { "No" });
}
