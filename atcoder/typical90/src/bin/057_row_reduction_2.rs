use std::iter;

use ac_library::pow_mod;
use proconio::{input, marker::Usize1};

fn xor(a: &[bool], b: &[bool]) -> Vec<bool> {
    let v: Vec<_> = iter::zip(a, b).map(|(&a, &b)| a ^ b).collect();
    v
}

fn f(mut a: Vec<Vec<bool>>, mut b: Vec<bool>) -> usize {
    let n = a.len();
    let m = b.len();

    let mut pos = 0; // pos 以降の行列を整える
    for i in 0..m {
        let Some(j) = (pos..n).find(|&j| a[j][i]) else {
            continue;
        };
        a.swap(pos, j);

        for j in 0..n {
            if j != pos && a[j][i] {
                a[j] = xor(&a[j], &a[pos]);
            }
        }
        pos += 1;
    }

    for a in &a[0..pos] {
        if let Some(i) = a.iter().position(|&x| x) {
            if b[i] {
                b = xor(a, &b);
            }
        }
    }

    let yes = b.iter().all(|&x| !x);
    if yes {
        let k = n - pos; // 残りはすべて 0。自由に選択できるはず
        let result = pow_mod(2, k as i64, 998244353) as usize;
        result
    } else {
        0
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut a = vec![vec![false; m]; n];
    for i in 0..n {
        input! {
            t: usize,
            a0: [Usize1; t],
        }
        for j in a0 {
            a[i][j] = true;
        }
    }
    input! {
        b: [usize; m],
    }
    let b: Vec<_> = b.iter().map(|&x| x == 1).collect();

    let result = f(a, b);
    println!("{result}");
}
