use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ta: [(u8, usize); n],
    }

    let mut state = vec![0usize; k + 1]; // state[荷物の数] = 最大スコア
    for &(t, a) in &ta {
        match t {
            0 => {
                state[0] = state[0].max(state[1]) + a;
                for i in 1..k {
                    state[i] = state[i + 1] + a;
                }
            }
            1 => {
                for i in (0..k).rev() {
                    state[i + 1] = state[i + 1].max(state[i] + a);
                }
            }
            _ => unreachable!(),
        }
    }
    let result = state[0];
    println!("{result}");
}
