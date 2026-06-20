use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        hl: [(usize, usize); n],
        q: usize,
        t: [usize; q],
    }

    // イベントソート
    let mut events = vec![];
    let mut set = BTreeSet::new();
    for (i, &(h, l)) in hl.iter().enumerate() {
        set.insert((h, i));
        events.push((l, 0, i)); // 時刻 l に i 番目の高橋君が抜ける
    }
    for (i, &t) in t.iter().enumerate() {
        events.push((t, 1, i)); // 時刻 t に i 番目の回答を更新する
    }
    events.sort();

    let mut results = vec![0usize; q];
    for &(_, k, i) in &events {
        if k == 0 {
            let (h, _) = hl[i];
            set.remove(&(h, i));
        } else {
            let (h, _) = *set.last().unwrap();
            results[i] = h;
        }
    }

    for &result in &results {
        println!("{result}");
    }
}
