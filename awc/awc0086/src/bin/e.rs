use proconio::{input, marker::Chars};

fn f(s: &[bool], t: &[bool]) -> usize {
    let mut state = vec![vec![0usize; t.len() + 1]; s.len() + 1];
    for (i, &s) in s.iter().enumerate() {
        for (j, &t) in t.iter().enumerate() {
            state[i + 1][j + 1] = state[i + 1][j].max(state[i][j + 1]);
            if s == t {
                state[i + 1][j + 1] = state[i + 1][j + 1].max(state[i][j] + 1);
            }
        }
    }
    state[s.len()][t.len()]
}

fn main() {
    input! {
        s: Chars,
        t: Chars,
        k: usize,
    }

    let s0 = s.iter().map(|&c| c == '1').collect::<Vec<_>>();
    let t0 = t.iter().map(|&c| c == '1').collect::<Vec<_>>();

    let mut result = 0;
    for i in 0..4 {
        let mut k0 = 0;
        let len = s0.len();
        let mut s0 = s0.clone();
        if (i & 1 != 0) && (len == 1 || s0[1] == s0[0]) {
            k0 += 1;
            s0[0] = !s0[0];
        }
        if (i & 2 != 0) && (len > 1 && s0[len - 2] == s0[len - 1]) {
            k0 += 1;
            s0[len - 1] = !s0[len - 1];
        }

        for j in 0..4 {
            let mut k0 = k0;
            let len = t0.len();
            let mut t0 = t0.clone();
            if (j & 1 != 0) && (len == 1 || t0[1] == t0[0]) {
                k0 += 1;
                t0[0] = !t0[0];
            }
            if (j & 2 != 0) && (len > 1 && t0[len - 2] == t0[len - 1]) {
                k0 += 1;
                t0[len - 1] = !t0[len - 1];
            }
            if k0 <= k {
                result = result.max(f(&s0, &t0));
            }
        }
    }

    println!("{result}");
}
