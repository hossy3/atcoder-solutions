use ac_library::Dsu;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
        }

        let n0 = 1usize << n;
        let mut v = vec![true; n0];
        for (i, &c) in s.iter().enumerate() {
            if c == '1' {
                v[i + 1] = false;
            }
        }

        let mut dsu = Dsu::new(n0);
        for i in 0..n0 {
            if !v[i] {
                continue;
            }
            for j in 0..n {
                let j0 = 1usize << j;
                let i0 = i | j0;
                if v[i0] && dsu.same(0, i) {
                    dsu.merge(i, i0);
                }
            }
        }
        let yes = dsu.same(0, n0 - 1);
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
