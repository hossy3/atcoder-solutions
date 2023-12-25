use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }
    let result = *[a * c, a * d, b * c, b * d].iter().max().unwrap();
    println!("{result}");
}
