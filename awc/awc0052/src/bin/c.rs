use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        vc: [(isize, usize); n],
    }

    let mut state = vec![isize::MIN; s + 1]; // 満足度
    state[0] = 0;
    for &(v, c) in &vc {
        if s < c {
            continue;
        }
        let prev = state.clone();
        for j in 0..=(s - c) {
            if prev[j] == isize::MIN {
                continue;
            }
            state[j + c] = state[j + c].max(prev[j] + v);
        }
    }

    let result = state[s];
    if result >= 0 {
        println!("{result}");
    } else {
        println!("-1");
    }
}
