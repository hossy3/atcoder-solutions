use proconio::{input, marker::Chars};

const N: usize = 2019;

fn main() {
    input! {
        s: Chars,
    }

    let mut a = [0isize; N];
    let mut cur = 0usize;
    let mut k = 1usize;
    for &c in s.iter().rev() {
        cur += k * (c as usize - '0' as usize);
        cur %= N;
        k *= 10;
        k %= N;
        a[cur] += 1;
    }
    let result = a.iter().map(|&x| x * (x - 1) / 2).sum::<isize>() + a[0];
    println!("{result}");
}
