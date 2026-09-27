use proconio::input;

fn main() {
    input! {
        n: usize,
        abs: [(usize, usize, String); n],
    }
    let result = abs
        .iter()
        .filter(|(_, _, s)| s.as_str() == "keep")
        .map(|(a, b, _)| b - a)
        .sum::<usize>();
    println!("{result}");
}
