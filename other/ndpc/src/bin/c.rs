use proconio::{input, marker::Chars};

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        n: usize,
        s: [Chars; 3],
    }

    let mut state = vec![vec![vec![Mint::new(0); s[2].len() + 1]; s[1].len() + 1]; s[0].len() + 1];
    state[0][0][0] += 1;
    for _ in 0..n {
        let prev = state;
        state = vec![vec![vec![Mint::new(0); s[2].len() + 1]; s[1].len() + 1]; s[0].len() + 1];

        for i0 in 0..=(s[0].len()) {
            for i1 in 0..=(s[1].len()) {
                for i2 in 0..=(s[2].len()) {
                    for c in 'a'..='z' {
                        let x = prev[i0][i1][i2];
                        let i0 = if i0 < s[0].len() && s[0][i0] == c {
                            i0 + 1
                        } else {
                            i0
                        };
                        let i1 = if i1 < s[1].len() && s[1][i1] == c {
                            i1 + 1
                        } else {
                            i1
                        };
                        let i2 = if i2 < s[2].len() && s[2][i2] == c {
                            i2 + 1
                        } else {
                            i2
                        };
                        state[i0][i1][i2] += x;
                    }
                }
            }
        }
    }

    let mut result = Mint::new(0);
    for i0 in 0..(s[0].len()) {
        for i1 in 0..(s[1].len()) {
            for i2 in 0..(s[2].len()) {
                result += state[i0][i1][i2];
            }
        }
    }
    println!("{result}");
}
