use std::ops::{BitOrAssign, Shr};

use bitset_fixed::BitSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        d: [isize; n],
        x: [isize; m],
    }

    if x.contains(&0) {
        println!("0"); // 移動できない
        return;
    }

    let x_minus_max = x
        .iter()
        .filter(|&&x| x < 0)
        .max()
        .cloned()
        .unwrap_or(isize::MIN);
    let x_plus_min = x
        .iter()
        .filter(|&&x| x > 0)
        .min()
        .cloned()
        .unwrap_or(isize::MAX);

    let d_min = d
        .iter()
        .map(|&d| d.min(0))
        .sum::<isize>()
        .max(x_minus_max + 1);
    let d_max = d
        .iter()
        .map(|&d| d.max(0))
        .sum::<isize>()
        .min(x_plus_min - 1);
    let mut bitset = BitSet::new((d_max - d_min + 1) as usize);
    eprintln!("{d_max} {d_min}");
    bitset.set(-d_min as usize, true);
    eprintln!("{:?}", &bitset);

    for &d in &d {
        if d > 0 {
            bitset.shl_or(d as usize);
        } else {
            bitset.bitor_assign(&bitset.clone().shr(-d as usize));
        }
    }
    for i in (0..=(d_max - d_min) as usize).rev() {
        if bitset[i] {
            println!("{}", i as isize + d_min);
            return;
        }
    }
}
