use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n - 1],
    }
    let result = -a.iter().sum::<i64>();
    println!("{result}");
}
