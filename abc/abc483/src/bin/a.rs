use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let result = n.div_ceil(2);
    println!("{result}");
}
