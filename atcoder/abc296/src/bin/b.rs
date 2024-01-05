use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 8],
    }
    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '*' {
                let c0 = 8 - i;
                let c1 = ('a' as u8 + j as u8) as char;
                println!("{}{}", c1, c0);
                return;
            }
        }
    }
}
