use proconio::{input, marker::Chars};

fn f(x: char, y: char) -> usize {
    if x >= y {
        x as usize - y as usize
    } else {
        10 + x as usize - y as usize
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    }

    let mut result = usize::MAX;
    for i in 0..=(n - m) {
        let mut value = 0;
        for j in 0..m {
            value += f(s[i + j], t[j]);
        }
        result = result.min(value);
    }
    println!("{result}");
}
