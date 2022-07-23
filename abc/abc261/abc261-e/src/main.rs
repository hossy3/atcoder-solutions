// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u32,
        c: u32,
        ta: [(u32, u32); n],
    }
    let mut x = c;
    let mut op_and = !0_u32;
    let mut op_or = 0_u32;
    let mut op_xor = 0_u32;
    for (t, a) in ta {
        if t == 1 {
            op_and &= a;
            op_or &= a;
            op_xor &= a;
        } else if t == 2 {
            op_and |= a;
            op_or |= a;
            op_xor &= !a;
        } else if t == 3 {
            op_xor ^= a;
        }
        x &= op_and;
        x |= op_or;
        x ^= op_xor;
        println!("{}", x);
    }
}
