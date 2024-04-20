use proconio::{input, marker::Chars};

fn f(s: &[char]) -> bool {
    let mut count = 0usize;
    for i in 0..3 {
        count += (s[i] as usize - '0' as usize) * 10usize.pow((2 - i) as u32);
    }
    0 < count && count < 350 && count != 316
}

fn main() {
    input! {
        s: Chars,
    }
    let yes = f(&s[3..]);
    println!("{}", if yes { "Yes" } else { "No" });
}
