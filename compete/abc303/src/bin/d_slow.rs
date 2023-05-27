use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

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
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0usize), 0, 0)); // ms, index, off/on
    let mut map = HashMap::new();

    let use_shift = y <= z + x + z;
    while let Some((Reverse(ms), i, caps)) = queue.pop() {
        if v[i][caps] < ms {
            continue;
        }
        v[i][caps] = ms;
        if i == n {
            println!("{}", ms);
            return;
        }
        if ms + z < v[i][1 - caps] {
            if let Some(&ms0) = map.get(&(i, 1 - caps)) {
                if ms + z < ms0 {
                    queue.push((Reverse(ms + z), i, 1 - caps));
                    map.insert((i, 1 - caps), ms + z);
                }
            } else {
                queue.push((Reverse(ms + z), i, 1 - caps));
                map.insert((i, 1 - caps), ms + z);
            }
        }
        if (caps == 0 && s[i] == 'a') || caps == 1 && s[i] == 'A' {
            if ms + x < v[i + 1][caps] {
                if let Some(&ms0) = map.get(&(i + 1, caps)) {
                    if ms + x >= ms0 {
                        continue;
                    }
                }
                queue.push((Reverse(ms + x), i + 1, caps));
                map.insert((i + 1, caps), ms + x);
            }
        } else {
            if use_shift && ms + y < v[i + 1][caps] {
                if let Some(&ms0) = map.get(&(i + 1, caps)) {
                    if ms + y >= ms0 {
                        continue;
                    }
                }
                queue.push((Reverse(ms + y), i + 1, caps));
                map.insert((i + 1, caps), ms + y);
            }
        }
    }
}
