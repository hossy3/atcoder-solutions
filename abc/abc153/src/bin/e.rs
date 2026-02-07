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
        for i in 0..h {
            if state[i] == usize::MAX {
                continue;
            }
            state[i + a] = state[i + a].min(state[i] + b);
        }
    }

    let result = *state[h..].iter().min().unwrap();
    println!("{result}");
}
