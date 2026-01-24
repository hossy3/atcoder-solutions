use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    let r: Vec<_> = s
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == 'R')
        .map(|(i, _)| i)
        .collect();
    let g: Vec<_> = s
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == 'G')
        .map(|(i, _)| i)
        .collect();
    let b: Vec<_> = s
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == 'B')
        .map(|(i, _)| i)
        .collect();

    let mut result = 0usize;
    for v in [r, g, b].iter().permutations(3) {
        for &i in v[0] {
            // v[1] の中で i より大きな番号を探す
            let j0 = v[1].partition_point(|&x| x < i);
            for &j in &v[1][j0..] {
                // v[2] の中で j より大きな番号を探す
                let k0 = v[2].partition_point(|&x| x < j);
                result += v[2].len() - k0;
                if v[2].binary_search(&(j * 2 - i)).is_ok() {
                    result -= 1; // j - i == k - j を除く
                }
            }
        }
    }

    println!("{result}");
}
