use itertools::Itertools;
use proconio::{input, marker::Chars};

fn to_index(c: char) -> usize {
    c as usize - 'a' as usize
}

fn update_indexes(indexes: &mut [Option<usize>], s: &[char], i: usize) {
    for c in 'a'..='z' {
        let c0 = to_index(c);
        let Some(j) = indexes[c0] else {
            continue;
        };
        if i < j {
            continue;
        }
        if let Some(y) = s[(i + 1)..].iter().position(|&x| x == c) {
            indexes[c0] = Some(i + 1 + y);
        } else {
            indexes[c0] = None;
        };
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut indexes = vec![];
    for c in 'a'..='z' {
        indexes.push(s.iter().position(|&x| x == c));
    }

    let mut results = vec![];
    for i in 0..k {
        for c in 'a'..='z' {
            let c0 = to_index(c);
            let Some(j) = indexes[c0] else {
                continue;
            };
            if n - j < k - i {
                continue;
            }
            results.push(c);
            update_indexes(&mut indexes, &s, j);
            break;
        }
    }

    let result = results.iter().join("");
    println!("{result}");
}
