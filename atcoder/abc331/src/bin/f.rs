use proconio::{
    input,
    marker::{Chars, Usize1},
};

type Mint = ac_library::ModInt998244353;
const N: usize = 26;

fn f(c: char) -> usize {
    c as usize - 'a' as usize
}

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
    }

    let mut powers = vec![Mint::new(1); n];
    for i in 0..(n - 1) {
        powers[i + 1] = powers[i] * N;
    }

    let mut bit0 = ac_library::FenwickTree::new(n, Mint::new(0));
    let mut bit1 = ac_library::FenwickTree::new(n, Mint::new(0));

    for (i, &c) in s.iter().enumerate() {
        let j = n - i - 1;
        bit0.add(i, powers[i] * f(c));
        bit1.add(j, powers[j] * f(c));
    }

    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                i: Usize1,
                c: char,
            }
            let j = n - i - 1;
            bit0.add(i, -bit0.sum(i..=i) + powers[i] * f(c));
            bit1.add(j, -bit1.sum(j..=j) + powers[j] * f(c));
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            }
            let (l1, r1) = (n - r - 1, n - l - 1);
            let hash0 = bit0.sum(l..=r) / powers[l];
            let hash1 = bit1.sum(l1..=r1) / powers[l1];
            let yes = hash0 == hash1;
            println!("{}", if yes { "Yes" } else { "No" });
        }
    }
}
