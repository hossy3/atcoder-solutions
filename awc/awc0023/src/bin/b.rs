use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
        c: usize,
    }

    let mut result = 0;
    for &(a, b) in &ab {
        result += a;
        result = result.saturating_sub(b);
    }
    result += c;
    println!("{result}");
}
