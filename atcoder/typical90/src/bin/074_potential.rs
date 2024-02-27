use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    let result: usize = s
        .iter()
        .enumerate()
        .map(|(i, c)| match c {
            'a' => 0,
            'b' => 1 << i,
            'c' => 2 << i,
            _ => unreachable!(),
        })
        .sum();
    println!("{result}");
}
