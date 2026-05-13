use proconio::input;

// 配るDP (Push DP)

fn main() {
    input! {
        n: usize,
        abc: [[usize; 3]; n],
    }

    let mut state = abc[0].clone();
    for abc in &abc[1..] {
        let mut new_state = vec![0; 3];
        for i in 0..3 {
            for j in 0..3 {
                if i == j {
                    continue;
                }
                new_state[i] = new_state[i].max(state[j] + abc[i]);
            }
        }
        state = new_state;
    }

    let result = state.iter().max().unwrap();
    println!("{result}");
}
