use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [usize; n],
        e: [[usize; m]; n],
    }

    let mut e0 = vec![0usize; n];
    for (i, e) in e.iter().enumerate() {
        for (j, &e) in e.iter().enumerate() {
            e0[i] |= e << j;
        }
    }

    let mut state = vec![usize::MAX; 1 << m];
    state[0] = 0;
    for s in 0..(1 << m) {
        if state[s] == usize::MAX {
            continue;
        }
        for (i, &e0) in e0.iter().enumerate() {
            let s0 = s | e0;
            state[s0] = state[s0].min(state[s] + c[i]);
        }
    }

    if state[(1 << m) - 1] < usize::MAX {
        println!("{}", state[(1 << m) - 1]);
    } else {
        println!("-1");
    }
}
