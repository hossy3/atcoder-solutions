use std::collections::BTreeMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        xy: [(Usize1, usize); q],
    }

    let mut a = vec![0; n];

    let mut mn = BTreeMap::new();
    mn.insert(0, n);

    let mut mk = BTreeMap::new();
    mk.insert(0, k);
    let mut sum = 0usize;

    for &(x, y) in &xy {
        let y0 = a[x];
        a[x] = y;

        if mn[&y0] > 1 {
            *mn.entry(y0).or_insert(0) -= 1;
        } else {
            mn.remove(&y0);
        }
        *mn.entry(y).or_insert(0) += 1;

        let y_min = *mk.iter().next().unwrap().0;
        if mk.contains_key(&y0) {
            if mk[&y0] > 1 {
                *mk.entry(y0).or_insert(0) -= 1;
            } else {
                mk.remove(&y0);
            }
            sum -= y0;

            if y_min <= y {
                *mk.entry(y).or_insert(0) += 1;
                sum += y;
            } else {
                if mn.get(&y_min).unwrap_or(&0) > mk.get(&y_min).unwrap_or(&0) {
                    *mk.entry(y_min).or_insert(0) += 1;
                    sum += y_min;
                } else {
                    let y1 = *mn.range(0..y_min).last().unwrap().0;
                    mk.insert(y1, 1);
                    sum += y1;
                }
            }
        } else {
            if y_min < y {
                if mk[&y_min] > 1 {
                    *mk.entry(y_min).or_insert(0) -= 1;
                } else {
                    mk.remove(&y_min);
                }
                *mk.entry(y).or_insert(0) += 1;
                sum += y - y_min;
            }
        }

        println!("{}", sum);
    }
}
