use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
    }
    let diff = x - y;
    let yes = (-2 <= diff) && (diff <= 3);
    println!("{}", if yes { "Yes" } else { "No" });
}
