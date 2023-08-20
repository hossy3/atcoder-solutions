use std::collections::HashSet;

use proconio::input;

// score: 4547731

fn main() {
    input! {
        n: usize,
        k: usize,
        l: usize,
        ab: [(usize, usize); k],
        c: [[usize; n]; n],
    }

    let mut vk = vec![(l, HashSet::new()); k]; // (i, conn) l=null
    for i in 0..n {
        for j in 0..n {
            let c0 = c[i][j];
            if c0 == 0 {
                continue;
            }
            if i > 0 {
                let c1 = c[i - 1][j];
                if c1 != 0 && c1 != c0 {
                    vk[c0 - 1].1.insert(c1 - 1);
                }
            }
            if i + 1 < n {
                let c1 = c[i + 1][j];
                if c1 != 0 && c1 != c0 {
                    vk[c0 - 1].1.insert(c1 - 1);
                }
            }
            if j > 0 {
                let c1 = c[i][j - 1];
                if c1 != 0 && c1 != c0 {
                    vk[c0 - 1].1.insert(c1 - 1);
                }
            }
            if j + 1 < n {
                let c1 = c[i][j + 1];
                if c1 != 0 && c1 != c0 {
                    vk[c0 - 1].1.insert(c1 - 1);
                }
            }
        }
    }

    let mut mabi = Vec::with_capacity(k);
    for (i, &(a, b)) in ab.iter().enumerate() {
        mabi.push((a * b, a, b, i));
    }
    mabi.sort();

    let mut vl = vec![(0, 0, HashSet::new()); l]; // (a, b, includes)
    let mut s = HashSet::<usize>::new();
    for j in 0..l {
        let &(_, a, b, i) = &mabi[j];
        vk[i].0 = j;
        vl[j] = (a, b, vk[i].1.clone());
        s.extend(vk[i].1.iter());
    }

    while let Some(&i) = s.iter().next() {
        s.remove(&i);
        if vk[i].0 < l {
            continue;
        }

        // merge
        let mut j = l;
        for &i0 in &vk[i].1 {
            let j0 = vk[i0].0;
            if j0 < l && (j == l || vl[j0].0 * vl[j0].1 < vl[j].0 * vl[j].1) {
                j = j0;
            }
        }
        assert_ne!(j, l);

        vk[i].0 = j;
        vl[j].0 += ab[i].0;
        vl[j].1 += ab[i].1;
        vl[j].2.extend(vk[i].1.iter());
        s.extend(vk[i].1.iter());
    }

    for &(i, _) in &vk {
        println!("{}", i + 1);
    }
}
