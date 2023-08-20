use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let k = 63 - n.leading_zeros();
    println!("{}", k);
}
