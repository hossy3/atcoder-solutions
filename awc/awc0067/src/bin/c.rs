use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        cf: [(usize, usize); n],
    }

    let mut state = vec![usize::MAX; d + 1];
    state[0] = 0;
    for &(c, f) in &cf {
        for i in 0..d {
            if state[i] == usize::MAX {
                continue;
            }
            let i0 = (i + f).min(d);
            state[i0] = state[i0].min(state[i] + c);
        }
        // eprintln!("{:?}", &state);
    }
    let result = state[d];
    println!("{result}");
}
