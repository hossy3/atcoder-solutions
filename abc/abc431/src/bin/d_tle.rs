use std::collections::BTreeMap;

use proconio::input;

fn f(w1: isize, score1: isize, next: &mut BTreeMap<isize, isize>) {
    if let Some((_, &score2)) = next.range(w1..).next() {
        if score2 >= score1 {
            return; // 何もしない
        }
    }
    next.insert(w1, score1);
    while let Some((&w2, &score2)) = next.range(..w1).last() {
        if score2 > score1 {
            break;
        }
        next.remove(&w2); // 不要になった候補を消す
    }
}

fn main() {
    input! {
        n: usize,
        whb: [(isize, isize, isize); n],
    }

    let mut state = BTreeMap::new();
    state.insert(0isize, 0isize);

    for &(w, h, b) in &whb {
        let mut next = BTreeMap::new();
        for (&w0, &score0) in &state {
            f(w0 - w, score0 + h, &mut next);
            f(w0 + w, score0 + b, &mut next);
        }

        state = next;
    }

    let Some((_, &result)) = state.range(0..).next() else {
        unreachable!();
    };
    println!("{result}");
}
