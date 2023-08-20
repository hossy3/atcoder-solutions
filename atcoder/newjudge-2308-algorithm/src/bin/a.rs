use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    }
    let result = &"atcoder"[(l - 1)..r];
    println!("{result}");
}
