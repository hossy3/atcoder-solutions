use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut i = s[0] as u8 - b'0';
    let mut j = s[2] as u8 - b'0';
    if j == 8 {
        i += 1;
        j = 1;
    } else {
        j += 1;
    }
    println!("{i}-{j}");
}
