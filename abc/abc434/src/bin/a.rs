use proconio::input;

fn main() {
    input! {
        w: usize,
        b: usize,
    }
    let result = (w * 1000 + 1).div_ceil(b);
    println!("{result}");
}
