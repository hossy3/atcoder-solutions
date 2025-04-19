use std::collections::{HashMap, HashSet};

use ac_library::Dsu;
use proconio::{input_interactive, marker::Usize1};

fn main() {
    input_interactive! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    }

    // 赤黒木を作る
    let mut set = HashSet::new();
    let mut dsu = Dsu::new(n * 2);
    for &(u, v) in &uv {
        dsu.merge(u, v + n);
        dsu.merge(u + n, v);
        set.insert((u.min(u), u.max(v)));
    }

    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        let x = dsu.leader(i);
        if let Some(v) = map.get_mut(&x) {
            v.push(i);
        } else {
            map.insert(x, vec![i]);
        }
    }
    let mut v = vec![];
    for (_, v0) in map {
        v.push(v0);
    }
    assert_eq!(v.len(), 2);

    if (v[0].len() * v[1].len() - (n - 1)) % 2 == 1 {
        println!("First");
        'outer: for &i in &v[0] {
            for &j in &v[1] {
                if !set.contains(&(i.min(j), i.max(j))) {
                    set.insert((i.min(j), i.max(j)));
                    let u0 = i + 1;
                    let v0 = j + 1;
                    let (u0, v0) = (u0.min(v0), u0.max(v0));
                    println!("{u0} {v0}");
                    break 'outer;
                }
            }
        }
    } else {
        println!("Second");
    }

    loop {
        input_interactive! {
            u0: isize,
            v0: isize,
        }
        if u0 == -1 && v0 == -1 {
            break;
        }
        let u0 = (u0 - 1) as usize;
        let v0 = (v0 - 1) as usize;
        set.insert((u0.min(v0), u0.max(v0)));

        'outer: for &i in &v[0] {
            for &j in &v[1] {
                if !set.contains(&(i.min(j), i.max(j))) {
                    set.insert((i.min(j), i.max(j)));
                    let u0 = i + 1;
                    let v0 = j + 1;
                    let (u0, v0) = (u0.min(v0), u0.max(v0));
                    println!("{u0} {v0}");
                    break 'outer;
                }
            }
        }
    }
}
