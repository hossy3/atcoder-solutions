use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    let c = (a - b).abs();
    let yes = c == 1 || c == 9;
    println!("{}", if yes { "Yes" } else { "No" });
}
