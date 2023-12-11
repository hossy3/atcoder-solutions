use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
    }
    let result = n / w;
    println!("{result}");
}
