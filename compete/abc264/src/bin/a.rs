use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    }
    let s = "atcoder";
    println!("{}", &s[(l - 1)..r]);
}
