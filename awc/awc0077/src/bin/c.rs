use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }

    let mut state = vec![usize::MAX; s + 1]; // [費用] = 最小個数
    state[0] = 0;

    for &a in &a {
        if a > s {
            continue;
        }
        for i in (0..=(s - a)).rev() {
            if state[i] < usize::MAX {
                state[i + a] = state[i + a].min(state[i] + 1);
            }
        }
    }

    if state[s] < usize::MAX {
        println!("{}", state[s]);
    } else {
        println!("-1");
    }
}
