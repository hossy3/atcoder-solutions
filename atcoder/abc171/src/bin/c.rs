use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut len = 1usize;
    let mut rest = n - 1;
    while rest >= 26usize.pow(len as u32) {
        rest -= 26usize.pow(len as u32);
        len += 1;
    }
    let mut v = vec![0usize; len];
    for i in 0..len {
        v[len - i - 1] = (rest / 26usize.pow(i as u32)) % 26;
    }
    let result = v.iter().map(|&i| ('a' as u8 + i as u8) as char).join("");
    println!("{result}");
}
