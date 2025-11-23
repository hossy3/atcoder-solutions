use proconio::input;

fn main() {
    input! {
        h: usize,
        b: usize,
    }
    let result = h.saturating_sub(b);
    println!("{result}");
}
