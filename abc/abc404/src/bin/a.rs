use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut v = vec![true; 26];
    for &c in &s {
        let i = c as usize - 'a' as usize;
        v[i] = false;
    }
    for i in 0..26 {
        if v[i] {
            let c = (i as u8 + 'a' as u8) as char;
            println!("{c}");
            return;
        }
    }
}
