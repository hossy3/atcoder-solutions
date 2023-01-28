use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }
    let yes = (a + b + c) == 0;
    println!("{}", if yes { "Yes" } else { "No" });
}
