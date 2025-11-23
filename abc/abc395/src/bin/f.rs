use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        mut ud: [(usize, usize); n],
    }

    let mut result = 0usize;

    // それぞれ差の条件を満たすようにする
    let mut set = BTreeSet::new();
    for i in 0..n {
        let mut height = ud[i].0;
        if i > 0 {
            height = height.min(ud[i - 1].0 + x);
        }
        if i < n - 1 {
            height = height.min(ud[i + 1].0 + x);
        }
        if height < ud[i].0 {
            set.insert((height, i));
        }
    }
    while let Some((height, i)) = set.pop_last() {
        if ud[i].0 <= height {
            continue;
        }
        result += ud[i].0 - height;
        ud[i].0 = height;
        if i > 0 && ud[i - 1].0 > height + x {
            set.insert((height + x, i - 1));
        }
        if i < n - 1 && ud[i + 1].0 > height + x {
            set.insert((height + x, i + 1));
        }
    }

    let mut set = BTreeSet::new();
    for i in 0..n {
        let mut height = ud[i].1;
        if i > 0 {
            height = height.min(ud[i - 1].1 + x);
        }
        if i < n - 1 {
            height = height.min(ud[i + 1].1 + x);
        }
        if height < ud[i].1 {
            set.insert((height, i));
        }
    }
    while let Some((height, i)) = set.pop_last() {
        if ud[i].1 <= height {
            continue;
        }
        result += ud[i].1 - height;
        ud[i].1 = height;
        if i > 0 && ud[i - 1].1 > height + x {
            set.insert((height + x, i - 1));
        }
        if i < n - 1 && ud[i + 1].1 > height + x {
            set.insert((height + x, i + 1));
        }
    }

    // 歯のかみ合わせを考える
    let Some(height) = ud.iter().map(|&(u, d)| u + d).min() else {
        unreachable!()
    };
    for &(u, d) in &ud {
        result += u + d - height;
    }

    println!("{result}");
}
