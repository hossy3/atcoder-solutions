use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        x: i64,
        s: Chars,
    }
    let mut result = x;
    for &c in &s {
        match c {
            'o' => {
                result += 1;
            }
            'x' => {
                result = 0.max(result - 1);
            }
            _ => unreachable!(),
        }
    }
    println!("{result}");
}
