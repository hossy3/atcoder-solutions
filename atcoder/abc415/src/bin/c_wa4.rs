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
        let mut dsu = Dsu::new(n0);
        for i in 0..n0 {
            if i > 0 && s[i - 1] == '1' {
                continue;
            }
            for j in 0..n {
                let j0 = 1usize << j;
                let i0 = i | j0;
                if s[i0 - 1] == '0' {
                    dsu.merge(i, i0);
                }
            }
        }
        let yes = s[n0 - 2] == '0' && dsu.same(0, n0 - 1);
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
