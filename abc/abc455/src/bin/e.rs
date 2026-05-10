use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    // ab が同じ数を調べる
    let mut count_ab = 0usize;
    {
        let mut diff = 0isize;
        let mut map = HashMap::new();
        map.insert(0, 1);
        for &s in &s {
            if s == 'A' {
                diff += 1;
            } else if s == 'B' {
                diff -= 1;
            }
            if let Some(count) = map.get_mut(&diff) {
                count_ab += *count;
                *count += 1;
            } else {
                map.insert(diff, 1);
            }
        }
    }

    // bc が同じ数を調べる
    let mut count_bc = 0usize;
    {
        let mut diff = 0isize;
        let mut map = HashMap::new();
        map.insert(0, 1);
        for &s in &s {
            if s == 'B' {
                diff += 1;
            } else if s == 'C' {
                diff -= 1;
            }
            if let Some(count) = map.get_mut(&diff) {
                count_bc += *count;
                *count += 1;
            } else {
                map.insert(diff, 1);
            }
        }
    }

    // ac が同じ数を調べる
    let mut count_ac = 0usize;
    {
        let mut diff = 0isize;
        let mut map = HashMap::new();
        map.insert(0, 1);
        for &s in &s {
            if s == 'A' {
                diff += 1;
            } else if s == 'C' {
                diff -= 1;
            }
            if let Some(count) = map.get_mut(&diff) {
                count_ac += *count;
                *count += 1;
            } else {
                map.insert(diff, 1);
            }
        }
    }

    // abc が同じ数を調べる
    let mut count_abc = 0usize;
    {
        let mut diff_ab = 0isize;
        let mut diff_ac = 0isize;
        let mut map = HashMap::new();
        map.insert((0, 0), 1);
        for &s in &s {
            if s == 'A' {
                diff_ab += 1;
                diff_ac += 1;
            } else if s == 'B' {
                diff_ab -= 1;
            } else if s == 'C' {
                diff_ac -= 1;
            }
            if let Some(count) = map.get_mut(&(diff_ab, diff_ac)) {
                count_abc += *count;
                *count += 1;
            } else {
                map.insert((diff_ab, diff_ac), 1);
            }
        }
    }

    // eprintln!("{count_ab} {count_bc} {count_ac} {count_abc}");
    let result = n * (n + 1) / 2 + count_abc * 2 - (count_ab + count_bc + count_ac);
    println!("{result}");
}
