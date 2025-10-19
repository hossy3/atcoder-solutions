use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        t: Chars,
    }
    let mut a = [0usize, 0usize];
    let mut cur = 0; // 0 の個数が偶数なら 0, 奇数なら 1
    let mut result = 0usize;
    for c in t {
        if c == '0' {
            cur = 1 - cur;
            result += a[cur];
            a[1 - cur] += 1;
        } else {
            a[cur] += 1;
            result += a[cur];
        }
    }
    println!("{result}");
}
