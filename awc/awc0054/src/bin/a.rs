use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let ctoi = |c: char| (c as u8 - b'a') as usize;
    let mut v = vec![0; 26];

    for s in s {
        v[ctoi(s[0])] += 1;
    }
    let result = v.into_iter().max().unwrap();
    println!("{result}");
}
