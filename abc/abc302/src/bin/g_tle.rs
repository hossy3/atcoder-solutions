use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut a0 = a.clone();
    a0.sort();
    let a0 = a0;

    let mut result = 0;
    'outer: loop {
        let mut map = HashMap::new();
        for i in 0..n {
            let x = a[i];
            let x0 = a0[i];
            if x != x0 {
                if let Some(&j) = map.get(&(x0, x)) {
                    a[i] = a0[i];
                    a[j] = a0[j];
                    result += 1;
                    continue 'outer;
                } else {
                    map.insert((x, x0), i);
                }
            }
        }
        if map.len() == 0 {
            break;
        }

        let mut it = map.iter();
        let (&(x, x0), &i) = it.next().unwrap();
        while let Some((&(y, y0), &j)) = it.next() {
            if x == y0 || x0 == y {
                a.swap(i, j);
                result += 1;
                continue 'outer;
            }
        }
        panic!();
    }

    println!("{}", result);
}
