use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut state = vec![vec![None; n + 1]; n + 1];
    for i in 0..=n {
        state[i][i] = Some([0, 0]);
    }

    for len in 1..=n {
        for i in 0..=n - len {
            let player = (n - len) % 2;
            let mut l0 = state[i][i + len - 1].unwrap();
            let mut r0 = state[i + 1][i + len].unwrap();
            l0[player] += a[i + len - 1];
            r0[player] += a[i];
            let x0 = if l0[player] > r0[player] { l0 } else { r0 };
            if let Some(x) = state[i][i + len] {
                if x[player] < x0[player] {
                    state[i][i + len] = Some(x0);
                }
            } else {
                state[i][i + len] = Some(x0);
            }
        }
    }

    let players = state[0][n].unwrap();
    println!("{} {}", players[0], players[1]);
}
