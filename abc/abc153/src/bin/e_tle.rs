use proconio::input;

fn main() {
    input! {
        h: usize,
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut state = vec![usize::MAX; 20001]; // 消費する魔力の最小値
    state[0] = 0;

    for &(a, b) in &ab {
        for i in (0..h).rev() {
            if state[i] == usize::MAX {
                continue;
            }
            for j in (i..h).step_by(a) {
                let count = (j - i) / a + 1;
                state[j + a] = state[j + a].min(state[i] + b * count);
            }
        }
    }

    let result = *state[h..].iter().min().unwrap();
    println!("{result}");
}
