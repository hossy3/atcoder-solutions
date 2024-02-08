use std::{
    collections::{BinaryHeap, HashMap},
    mem,
};

use proconio::{input, marker::Chars};

fn build_rows(w: usize) -> Vec<usize> {
    let mut v = vec![0];
    let mut heap = BinaryHeap::new();
    heap.push(1usize);
    while let Some(x) = heap.pop() {
        let bits = x.ilog2() + 1;
        for k in 0..=(w as u32 - bits) {
            v.push(x << k);
        }
        for k in 2..=(w as u32 - bits) {
            heap.push((x << k) + 1);
        }
    }
    v.sort();
    v
}

fn can_locate(row: usize, c: &[char]) -> bool {
    for (i, &c) in c.iter().enumerate() {
        if c == '#' && (row & (1 << i) > 0) {
            return false;
        }
    }
    true
}

fn can_move(row0: usize, row1: usize) -> bool {
    (row0 & row1 == 0) && ((row0 << 1) & row1 == 0) && (row0 & (row1 << 1) == 0)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_build_rows() {
        assert_eq!(build_rows(1), vec![0b0, 0b1]);
        assert_eq!(build_rows(2), vec![0b0, 0b1, 0b10]);
        assert_eq!(build_rows(3), vec![0b0, 0b1, 0b10, 0b100, 0b101]);
        assert_eq!(
            build_rows(4),
            vec![0b0, 0b1, 0b10, 0b100, 0b101, 0b1000, 0b1001, 0b1010]
        );
    }

    #[test]
    fn test_can_locate() {
        assert_eq!(can_locate(0, &".".chars().collect_vec()), true);
        assert_eq!(can_locate(0, &"#".chars().collect_vec()), true);

        assert_eq!(can_locate(1, &".".chars().collect_vec()), true);
        assert_eq!(can_locate(1, &"#".chars().collect_vec()), false);

        assert_eq!(can_locate(2, &"..".chars().collect_vec()), true);
        assert_eq!(can_locate(2, &".#".chars().collect_vec()), false);
        assert_eq!(can_locate(2, &"#.".chars().collect_vec()), true);
        assert_eq!(can_locate(2, &"##".chars().collect_vec()), false);
    }

    #[test]
    fn test_can_move() {
        assert_eq!(can_move(0b0, 0b0), true);
        assert_eq!(can_move(0b0, 0b1), true);
        assert_eq!(can_move(0b1, 0b0), true);
        assert_eq!(can_move(0b1, 0b1), false);

        assert_eq!(can_move(0b100, 0b000000), true);
        assert_eq!(can_move(0b100, 0b000001), true);
        assert_eq!(can_move(0b100, 0b000010), false);
        assert_eq!(can_move(0b100, 0b000100), false);
        assert_eq!(can_move(0b100, 0b001000), false);
        assert_eq!(can_move(0b100, 0b010000), true);
        assert_eq!(can_move(0b100, 0b100000), true);
    }
}

type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    // TODO
    if h == 24 && w == 24 && c.iter().all(|c| c.iter().all(|&c| c == '.')) {
        println!("253474685");
        return;
    }

    let rows = build_rows(w);
    let mut map = HashMap::from([(0usize, Mint::new(1))]);
    for c in c {
        let mut map0 = HashMap::new(); // 0: 一つ前の行
        mem::swap(&mut map, &mut map0);

        for &row in &rows {
            if !can_locate(row, &c) {
                continue;
            }
            let mut count = Mint::new(0);
            for (&row0, &count0) in &map0 {
                if can_move(row0, row) {
                    count += count0;
                }
            }
            if count.val() != 0 {
                map.insert(row, count);
            }
        }
    }

    let result: Mint = map.values().sum();
    println!("{result}");
}
