use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn f(c: char) -> usize {
    c as usize - 'a' as usize
}

fn g(c: &[Vec<char>]) -> usize {
    let h = c.len();
    let w = c[0].len();

    let mut rows = vec![vec![0; 26]; h]; // [i][x] = count
    let mut cols = vec![vec![0; 26]; w];
    let mut rows0 = vec![vec![vec![]; 26]; h]; // [i][x] = vec![j, ...]
    let mut cols0 = vec![vec![vec![]; 26]; w];
    for i in 0..h {
        for j in 0..w {
            let x = f(c[i][j]);
            rows[i][x] += 1;
            cols[j][x] += 1;
            rows0[i][x].push(j);
            cols0[j][x].push(i);
        }
    }

    let mut rows1 = vec![0; h]; // [i] = count of alphabet type
    let mut cols1 = vec![0; w];
    for i in 0..h {
        rows1[i] = rows[i].iter().filter(|&&x| x > 0).count();
    }
    for j in 0..w {
        cols1[j] = cols[j].iter().filter(|&&x| x > 0).count();
    }

    let mut rows2 = vec![w; h]; // [i] = count
    let mut cols2 = vec![h; w];

    loop {
        let mut modified = HashSet::new();
        for i in 0..h {
            if rows1[i] != 1 || rows2[i] < 2 {
                continue;
            }
            let x = rows[i].iter().position(|&x| x > 0).unwrap();
            for &j in &rows0[i][x] {
                if cols[j][x] > 0 {
                    modified.insert((i, j, x));
                } 
            }
        }

        for j in 0..w {
            if cols1[j] != 1 || cols2[j] < 2 {
                continue;
            }
            let x = cols[j].iter().position(|&x| x > 0).unwrap();
            for &i in &cols0[j][x] {
                if rows[i][x] > 0 {
                    modified.insert((i, j, x));
                } 
            }
        }

        if modified.len() == 0 {
            break;
        }

        for (i, j, x) in modified {
            rows[i][x] -= 1;
            if rows[i][x] == 0 {
                rows1[i] -= 1;
            }
            rows2[i] -= 1;

            cols[j][x] -= 1;
            if cols[j][x] == 0 {
                cols1[j] -= 1;
            }
            cols2[j] -= 1;
        }
    }

    let result: usize = rows.iter().map(|v| v.iter().sum::<usize>()).sum();
    result
}

#[test]
fn test_func() {
    use itertools::Itertools;
    assert_eq!(
        g(&["aaaaa".chars().collect_vec(), "abcde".chars().collect_vec()]),
        4
    );
    assert_eq!(
        g(&[
            "zbz".chars().collect_vec(),
            "zaz".chars().collect_vec(),
            "zbz".chars().collect_vec(),
        ]),
        3
    );
    assert_eq!(
        g(&[
            "aaa".chars().collect_vec(),
            "cbb".chars().collect_vec(),
            "aab".chars().collect_vec(),
            "aab".chars().collect_vec(),
        ]),
        2
    );
    assert_eq!(
        g(&[
            "aa".chars().collect_vec(),
            "bb".chars().collect_vec(),
            "ab".chars().collect_vec(),
            "ab".chars().collect_vec(),
        ]),
        0
    );
}

fn main() {
    input! {
        h: usize,
        _: usize,
        mut c: [Chars; h],
    }

    let result = g(&c);
    println!("{result}");
}
