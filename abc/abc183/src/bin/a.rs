use proconio::input;

fn main() {
    input! {
        x: i64,
    }
    let result = x.max(0);
    println!("{result}");
}
