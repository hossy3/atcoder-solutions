use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let result = if a >= b { 0 } else { (b - a) * c };
    println!("{result}");
}
