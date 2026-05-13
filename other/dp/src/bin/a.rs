use proconio::input;

// 配るDP (Push DP)

fn push_dp<S>(range: std::ops::Range<usize>, e: S, init: S, f: impl Fn(usize, &mut [S])) -> S
where
    S: Clone,
{
    let len = range.end + 1;
    let mut state = vec![e; len];
    state[0] = init;
    for i in range {
        f(i, &mut state);
    }
    let result = state[len - 1].clone();
    result
}

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }
    let result = push_dp(0..(n - 1), usize::MAX, 0, |i, state| {
        if state[i] < usize::MAX {
            state[i + 1] = state[i + 1].min(state[i] + h[i].abs_diff(h[i + 1]));
            if i + 2 < n {
                state[i + 2] = state[i + 2].min(state[i] + h[i].abs_diff(h[i + 2]));
            }
        }
    });
    println!("{result}");
}
