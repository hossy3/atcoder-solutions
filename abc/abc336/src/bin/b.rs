use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let result = n.trailing_zeros();
    println!("{result}");
}
