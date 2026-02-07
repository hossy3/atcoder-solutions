use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            c: Usize1,
            mut s: [Chars; n],
        }

        let mut checked = vec![false; n]; // 掘り進められることを確認済みか

        // 順に移動可能な場所を調べる
        let mut state = vec![false; n];
        state[c] = true;
        for i in (1..n).rev() {
            let mut state0 = vec![false; n];
            for j in 0..n {
                if !state[j] {
                    continue;
                }

                // 掘り進められるところを事前に解決する
                if !checked[j] {
                    checked[j] = true;
                    if (i..n).all(|i| s[i][j] == '.') {
                        for i in 0..i {
                            s[i][j] = '.';
                        }
                    }
                }
                if j > 0 && !checked[j - 1] {
                    let j = j - 1;
                    checked[j] = true;
                    if (i..n).all(|i| s[i][j] == '.') {
                        for i in 0..i {
                            s[i][j] = '.';
                        }
                    }
                }
                if j < n - 1 && !checked[j + 1] {
                    let j = j + 1;
                    checked[j] = true;
                    if (i..n).all(|i| s[i][j] == '.') {
                        for i in 0..i {
                            s[i][j] = '.';
                        }
                    }
                }

                if s[i - 1][j] == '.' {
                    state0[j] = true;
                }
                if j > 0 && s[i - 1][j - 1] == '.' {
                    state0[j - 1] = true;
                }
                if j < n - 1 && s[i - 1][j + 1] == '.' {
                    state0[j + 1] = true;
                }
            }
            state = state0;
        }

        let result = state
            .iter()
            .map(|&b| if b { '1' } else { '0' })
            .collect::<String>();
        println!("{result}");
    }
}
