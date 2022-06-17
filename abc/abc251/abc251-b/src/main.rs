// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut result = vec![0; w + 1];

    let n = match a.binary_search(&w) {
        Ok(n) | Err(n) => n,
    };
    for i in 0..n {
        let x = a[i];
        if x <= w {
            result[x] = 1;
        }
        for j in (i + 1)..n {
            let x = a[i] + a[j];
            if x <= w {
                result[x] = 1;
            }
            for k in (j + 1)..n {
                let x = a[i] + a[j] + a[k];
                if x <= w {
                    result[x] = 1;
                }
            }
        }
    }
    let sum: i32 = result.iter().sum();
    println!("{}", sum);
}
