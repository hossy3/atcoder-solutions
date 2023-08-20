use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v = vec![BTreeSet::new(); 200];
    for (i, &x) in a.iter().enumerate() {
        let x0 = x % 200;
        if v[x0].len() > 0 || v[0].len() > 0 {
            if x0 != 0 && v[0].len() > 0 {
                v[x0] = v[0].clone();
                v[x0].insert(i + 1);
            }
            let s0 = &v[x0];
            println!("Yes");
            println!("{} {}", s0.len(), s0.iter().join(" "));
            println!("{} {}", 1, i + 1);
            return;
        }

        let v0 = v.clone();
        v[x0].insert(i + 1);

        for j in 0..200 {
            if v0[j].len() == 0 {
                continue;
            }

            let x1 = (j + x) % 200;
            let s1 = &v0[x1];
            if s1.len() > 0 {
                let mut s2 = v0[j].clone();
                s2.insert(i + 1);

                println!("Yes");
                println!("{} {}", s1.len(), s1.iter().join(" "));
                println!("{} {}", s2.len(), s2.iter().join(" "));
                return;
            }

            v[x1] = v0[j].clone();
            v[x1].insert(i + 1);
        }
    }

    println!("No");
}
