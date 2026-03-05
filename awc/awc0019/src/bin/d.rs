use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        t: usize,
        abc: [(isize, usize, usize); n],
    }

    let mut state = vec![isize::MIN; m + 1]; // state[費用] = 最大価値
    state[0] = 0;
    for &(a, b, c) in &abc {
        for j in (0..=m).rev() {
            if b < t {
                if j >= c {
                    state[j] = state[j].max(state[j - c] + a);
                }
            } else {
                if state[j] > isize::MIN {
                    state[j] += a;
                }
            }
        }
    }

    let result = *state.iter().max().unwrap();
    println!("{result}");
}
