use proconio::input;

fn main() {
    input! {
        x: usize,
    }
    let yes = 1 * 3 <= x && x <= 6 * 3;
    println!("{}", if yes { "Yes" } else { "No" });
}
