use proconio::input;

fn main() {
    input! {
        x: i64,
    }
    let yes = x >= 30;
    println!("{}", if yes { "Yes" } else { "No" });
}
