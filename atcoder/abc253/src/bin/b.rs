// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [String; h],
    }
    let mut positions: [(i32, i32); 2] = [(0, 0), (0, 0)];
    let mut i = 0;
    for y in 0..h {
        let s0 = &s[y];
        for x in 0..w {
            let c = &s0[x..=x];
            if c == "o" {
                positions[i].0 = x as i32;
                positions[i].1 = y as i32;
                i += 1;
            }
        }
    }
    let result = (positions[0].0 - positions[1].0).abs() + (positions[0].1 - positions[1].1).abs();
    println!("{}", result);
}
