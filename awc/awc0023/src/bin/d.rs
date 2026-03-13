use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        t: usize,
        pcw: [(usize, usize, usize); n],
    }

    let mut state = vec![vec![usize::MAX; n + 1]; s + 1]; // state[重さ][荷物の数] = 最大価値
    state[0][0] = 0;

    for (i, &(p, c, w)) in pcw.iter().enumerate() {
        if p <= c {
            continue; // 利益のない荷物は運ばない
        }
        if w > s {
            continue; // 重さの合計が s を超えない。1つで超えてしまう場合は運ばない
        }
        for j in (0..=(s - w)).rev() {
            for i0 in 0..=i {
                if state[j][i0] == usize::MAX {
                    continue;
                }
                if state[j + w][i0 + 1] == usize::MAX {
                    state[j + w][i0 + 1] = state[j][i0] + p - c;
                } else {
                    state[j + w][i0 + 1] = state[j + w][i0 + 1].max(state[j][i0] + p - c);
                }
            }
        }
    }

    for i0 in 1..=n {
        for j in 0..=s {
            if state[j][i0] != usize::MAX && state[j][i0] >= t {
                println!("{i0}");
                return;
            }
        }
    }

    println!("-1");
}
