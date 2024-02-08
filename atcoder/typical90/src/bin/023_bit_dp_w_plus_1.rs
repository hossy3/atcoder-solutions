use proconio::{input, marker::Chars};
use std::{collections::HashMap, mem};

trait BitTest {
    fn bit_test(&self, i: usize) -> bool;
}

impl BitTest for usize {
    fn bit_test(&self, i: usize) -> bool {
        self & (1 << i) != 0
    }
}

fn can_move(w: usize, i: usize, state: usize) -> bool {
    if state.bit_test(w - 1) {
        return false; // 上にコマがある
    }
    if i > 0 && (state.bit_test(w) || state.bit_test(0)) {
        return false; // 左上または左にコマがある
    }
    if i < w - 1 && state.bit_test(w - 2) {
        return false; // 右上にコマがある
    }
    true
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_can_move() {
        assert_eq!(can_move(2, 0, 0b000), true);
        assert_eq!(can_move(2, 0, 0b001), false);
        assert_eq!(can_move(2, 0, 0b010), false);
        assert_eq!(can_move(2, 0, 0b011), false);
        assert_eq!(can_move(2, 0, 0b100), true);
        assert_eq!(can_move(2, 0, 0b101), false);
        assert_eq!(can_move(2, 0, 0b110), false);
        assert_eq!(can_move(2, 0, 0b111), false);

        assert_eq!(can_move(2, 1, 0b000), true);
        assert_eq!(can_move(2, 1, 0b001), false);
        assert_eq!(can_move(2, 1, 0b010), false);
        assert_eq!(can_move(2, 1, 0b011), false);
        assert_eq!(can_move(2, 1, 0b100), false);
        assert_eq!(can_move(2, 1, 0b101), false);
        assert_eq!(can_move(2, 1, 0b110), false);
        assert_eq!(can_move(2, 1, 0b111), false);
    }
}

type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let mask = (1 << (w + 1)) - 1;
    let mut states = HashMap::from([(0usize, Mint::new(1))]); // w+1 マス分の履歴を覚える
    for c in c {
        for (i, &c) in c.iter().enumerate() {
            let mut prev_states = HashMap::new();
            mem::swap(&mut states, &mut prev_states);

            for (prev_state, count) in prev_states {
                let state = (prev_state << 1) & mask;
                *states.entry(state).or_insert(Mint::new(0)) += count;
                if c != '#' && can_move(w, i, prev_state) {
                    *states.entry(state + 1).or_insert(Mint::new(0)) += count;
                }
            }
        }
    }

    let result: Mint = states.values().sum();
    println!("{result}");
}
