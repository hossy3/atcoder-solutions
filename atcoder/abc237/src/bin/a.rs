use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    const X: i64 = 1 << 31;
    let yes = -X <= n && n < X;
    println!("{}", if yes { "Yes" } else { "No" });
}
