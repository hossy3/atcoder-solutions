use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        k: usize,
        s: Chars,
    }
    let mut count = 0;
    let mut result = vec![];
    for c in s {
        if c == 'o' && count < k {
            count += 1;
            result.push("o");
        } else {
            result.push("x");
        }
    }
    let result = result.join("");
    println!("{}", result);
}
