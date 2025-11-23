use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut a = [0usize; 26];
    for c in s {
        let i = c as usize - 'a' as usize;
        a[i] += 1;
    }

    let mut max = 0;
    let mut result = 'a';
    for (i, &count) in a.iter().enumerate() {
        if count > max {
            max = count;
            result = (i as u8 + 'a' as u8) as char;
        }
    }

    println!("{result}");
}
