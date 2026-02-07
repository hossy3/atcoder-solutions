use proconio::input;

fn main() {
    input! {
        h: usize,
        a: usize,
    }
    let result = h.div_ceil(a);
    println!("{result}");
}
