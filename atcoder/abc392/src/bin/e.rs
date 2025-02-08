use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut dsu = Dsu::new(n);
    let mut cables = vec![]; // 冗長なケーブル
    for (i, &(a, b)) in ab.iter().enumerate() {
        if dsu.same(a, b) {
            cables.push((i, a, b));
        } else {
            dsu.merge(a, b);
        }
    }
    cables.reverse();

    let mut results = vec![];
    let mut i = 0; // 次につなぐ候補
    while let Some((j, a, _b)) = cables.pop() {
        while i < n && dsu.same(i, a) {
            i += 1;
        }
        if i == n {
            break;
        }
        results.push((j, a, i));
        dsu.merge(a, i);
    }

    println!("{}", results.len());
    for &(i, a, b) in &results {
        println!("{} {} {}", i + 1, a + 1, b + 1);
    }
}
