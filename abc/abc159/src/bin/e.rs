use proconio::{input, marker::Chars};

fn f(k: usize, s: &[Vec<char>], v: &[Vec<usize>]) -> Option<usize> {
    let w = s[0].len();

    let mut count = 0; // 追加する仕切り線の数
    let mut state = vec![0usize; v.len()];
    for i in 0..w {
        let mut state0 = vec![0usize; v.len()];
        for (j, v) in v.iter().enumerate() {
            for &row in v {
                if s[row][i] == '1' {
                    state0[j] += 1;
                }
            }
        }
        if state0.iter().any(|&x| x > k) {
            return None; // 幅1でも条件を満たせない
        }
        for j in 0..v.len() {
            state[j] += state0[j];
        }
        if state.iter().any(|&x| x > k) {
            count += 1;
            state = state0;
        }
    }

    Some(count)
}

fn main() {
    input! {
        h: usize,
        _: usize,
        k: usize,
        s: [Chars; h],
    }

    let h0 = 1 << (h - 1);
    let mut result = usize::MAX;
    for i in 0..h0 {
        let mut v = vec![]; // 1 で行を区切る
        v.push(vec![0]);
        for j in 0..(h - 1) {
            if i & (1 << j) == 0 {
                let x = v.len() - 1;
                v[x].push(j + 1);
            } else {
                v.push(vec![j + 1]);
            }
        }

        if let Some(x) = f(k, &s, &v) {
            result = result.min(x + v.len() - 1);
        }
    }
    println!("{result}");
}
