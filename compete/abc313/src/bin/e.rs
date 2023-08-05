use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    let mut v = vec![];
    for &c in &s {
        v.push(c as usize - '0' as usize);
    }
    for v in v.windows(2) {
        if v[0] > 1 && v[1] > 1 {
            println!("{}", -1);
            return;
        }
    }
    const MOD: usize = 998244353;
    let mut count = 0usize;
    for &x in v[1..].iter().rev() {
        count += 1;
        count += (x - 1) * count;
        count %= MOD;
    }
    println!("{}", count);
}
