use proconio::{input, marker::Chars};

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        s: Chars,
    }

    let mut cum = vec![0usize; s.len()]; // 同じ文字が現れる個数
    for i in 1..(s.len()) {
        cum[i] = cum[i - 1];
        if s[i] == s[i - 1] {
            cum[i] += 1;
        }
    }

    let mut result = Mint::new(0);
    for l in 0..cum.len() {
        let r = cum.partition_point(|&x| x <= cum[l]);
        result += r - l;
    }
    println!("{result}");
}
