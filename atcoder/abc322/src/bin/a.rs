use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }
    for (i, v) in s.windows(3).enumerate() {
        if v[0] == 'A' && v[1] == 'B' && v[2] == 'C' {
            let result = i + 1;
            println!("{result}");
            return;
        } 
    }
    println!("-1");
}
