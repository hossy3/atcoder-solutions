use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }
    let mut set = HashSet::<(usize, usize)>::new();
    for (u, v) in uv {
        set.insert((u - 1, v - 1));
    }
    let mut count = 0;
    for i in 0..(n - 2) {
        for j in (i + 1)..(n - 1) {
            if !set.contains(&(i, j)) {
                continue;
            }
            for k in (j + 1)..n {
                if !set.contains(&(j, k)) {
                    continue;
                }
                if !set.contains(&(i, k)) {
                    continue;
                }
                count += 1;
            }
        }
    }
    
    println!("{}", count);
}
