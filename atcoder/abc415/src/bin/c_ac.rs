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
        let mut v = vec![false; n0];
        v[0] = true;
        for i in 0..n0 {
            if !v[i] {
                continue;
            }
            for j in 0..n {
                let j0 = 1usize << j;
                let i0 = i | j0;
                if s[i0 - 1] == '0' {
                    v[i0] = true;
                }
            }
        }
        let yes = v[n0 - 1];
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
