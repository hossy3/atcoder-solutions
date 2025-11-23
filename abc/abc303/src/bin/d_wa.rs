use std::collections::BTreeMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        s: Chars,
    }
    let n = s.len();
    let mut v = vec![[std::usize::MAX, std::usize::MAX]; n + 1]; // [caps off, on]
    let mut map = BTreeMap::new();
    map.insert((0, 0), 0usize); // ms, index, off/on

    while let Some((&(i, caps), &ms)) = map.iter().next() {
        map.remove(&(i, caps));
        if v[i][caps] < ms {
            continue;
        }
        v[i][caps] = ms;
        if i == n {
            println!("{}", ms);
            return;
        }
        if ms + z < v[i][1 - caps] {
            let ms = ms + z;
            if let Some(&ms0) = map.get(&(i, 1 - caps)) {
                if ms < ms0 {
                    map.insert((i, 1 - caps), ms);
                }
            } else {
                map.insert((i, 1 - caps), ms);
            }
        }
        if (caps == 0 && s[i] == 'a') || caps == 1 && s[i] == 'A' {
            let ms = ms + x;
            if let Some(&ms0) = map.get(&(i + 1, caps)) {
                if ms < ms0 {
                    map.insert((i + 1, caps), ms);
                }
            } else {
                map.insert((i + 1, caps), ms);
            }
        } else {
            let ms = ms + y;
            if let Some(&ms0) = map.get(&(i + 1, caps)) {
                if ms < ms0 {
                    map.insert((i + 1, caps), ms);
                }
            } else {
                map.insert((i + 1, caps), ms);
            }
        }
    }
}
