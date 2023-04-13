// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [usize; 9],
    }
    let mut a = vec![0; 1_000_001];
    let mut rest = n;

    let min_c = c.iter().min().unwrap();
    let max_keta = n / min_c;
    'outer: for i in 0..=max_keta {
        for j in 0..9 {
            let cj = c[(8 - j) as usize];
            if (rest >= cj) && (rest - cj) / min_c >= (max_keta - i - 1) {
                rest -= cj;
                a[i] = 9 - j;
                continue 'outer;
            }
        }

        for j in 0..i {
            print!("{}", a[j]);
        }
        println!();
        break;
    }
}
