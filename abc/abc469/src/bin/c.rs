use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut results = vec![];
    for (i, &c) in s.iter().enumerate() {
        if c == 'x' {
            results.push(i + 1);
        }
    }
    let len = results.len();
    results.append(&mut vec![n; n - len]);
    for result in results {
        println!("{result}");
    }
}
