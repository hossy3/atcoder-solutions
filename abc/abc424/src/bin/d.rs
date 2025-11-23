use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            h: usize,
            w: usize,
            s: [Chars; h],
        }

        let mut state = vec![u32::MAX; 1 << w];
        state[0] = 0;

        let k_mask = (1 << (w - 1)) - 1; // マスク

        for s in &s {
            let mut state0 = vec![u32::MAX; 1 << w];
            let k0 = s.iter().enumerate().fold(
                0,
                |acc, (i, &c)| {
                    if c == '#' {
                        acc | (1 << i)
                    } else {
                        acc
                    }
                },
            );
            for k in 0..(1 << w) {
                if k0 | k != k0 {
                    continue; // もともとの白マスが黒マスになってはいけない
                }
                for (k1, &count) in state.iter().enumerate() {
                    if count == u32::MAX {
                        continue;
                    }
                    let k2 = k & k1;
                    // 黒マスが隣り合わなければ k_mask 内が空になるはず
                    if (k2 & (k2 >> 1)) & k_mask == 0 {
                        let count = count + k0.count_ones() - k.count_ones();
                        state0[k] = state0[k].min(count);
                    }
                }
            }

            state = state0;
        }

        let result = state.iter().min().unwrap();
        println!("{result}");
    }
}
