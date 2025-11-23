use proconio::{input, marker::Chars};

fn g(i: usize, j: usize, i_max: usize, j_max: usize, r: usize) -> (usize, usize) {
    match r {
        0 => (i, j),
        1 => (j, i_max.wrapping_sub(i)),
        2 => (i_max.wrapping_sub(i), j_max.wrapping_sub(j)),
        3 => (j_max.wrapping_sub(j), i),
        _ => unreachable!(),
    }
}

#[test]
fn test_func_g() {
    assert_eq!(g(0, 0, 1, 2, 0), (0, 0));
    assert_eq!(g(0, 0, 1, 2, 1), (0, 1));
    assert_eq!(g(0, 0, 1, 2, 2), (1, 2));
    assert_eq!(g(0, 0, 1, 2, 3), (2, 0));

    assert_eq!(g(1, 0, 1, 2, 0), (1, 0));
    assert_eq!(g(1, 0, 1, 2, 1), (0, 0));
    assert_eq!(g(1, 0, 1, 2, 2), (0, 2));
    assert_eq!(g(1, 0, 1, 2, 3), (2, 1));

    assert_eq!(g(2, 0, 1, 2, 0), (2, 0));
    assert_eq!(g(2, 0, 1, 2, 1), (0, usize::MAX));
    assert_eq!(g(2, 0, 1, 2, 2), (usize::MAX, 2));
    assert_eq!(g(2, 0, 1, 2, 3), (2, 2));
}

#[test]
fn test_func_f() {
    let p = vec![
        vec!['.', '.', '.', '.'],
        vec!['#', '#', '#', '.'],
        vec!['.', '#', '.', '.'],
        vec!['.', '.', '.', '.'],
        vec!['.', '.', '.', '.'],
        vec!['.', '#', '#', '#'],
        vec!['.', '#', '#', '.'],
        vec!['.', '.', '.', '.'],
        vec!['.', '.', '#', '.'],
        vec!['.', '#', '#', '.'],
        vec!['.', '#', '#', '.'],
        vec!['.', '#', '#', '.'],
    ];
    let m = vec![
        true, true, true, false, false, true, false, false, false, false, false, false, false,
        false, false, false,
    ];
    assert_eq!(f(1, &p, &m), true);
}

fn f(k: usize, p: &[Vec<char>], m: &Vec<bool>) -> bool {
    let mut v = vec![];
    for i in 0..4 {
        for j in 0..4 {
            if p[k * 4 + i][j] == '#' {
                v.push((i, j));
            }
        }
    }
    let i_min = v.iter().map(|&(i, _)| i).min().unwrap();
    let j_min = v.iter().map(|&(_, j)| j).min().unwrap();
    for i in 0..(v.len()) {
        v[i] = (v[i].0 - i_min, v[i].1 - j_min);
    }
    let i_max = v.iter().map(|&(i, _)| i).max().unwrap();
    let j_max = v.iter().map(|&(_, j)| j).max().unwrap();

    for i0 in 0..=3 {
        for j0 in 0..=3 {
            'outer: for r in 0..4 {
                if k == 1 && i0 == 1 && j0 == 0 && r == 3 {
                    eprintln!();
                }
                let mut m0 = m.clone();
                for &(i, j) in &v {
                    let (i, j) = g(i, j, i_max, j_max, r);
                    let (i, j) = (i.wrapping_add(i0), j.wrapping_add(j0));
                    if i >= 4 || j >= 4 {
                        continue 'outer;
                    }
                    if m0[i * 4 + j] {
                        continue 'outer;
                    }
                    m0[i * 4 + j] = true;
                }
                if k == 2 || f(k + 1, p, &m0) {
                    return true;
                }
            }
        }
    }

    false
}

fn main() {
    input! {
        p: [Chars; 12],
    }
    if p.iter()
        .map(|v| v.iter().filter(|x| **x == '#').count())
        .sum::<usize>()
        != 16
    {
        println!("No");
        return;
    }

    let v = vec![false; 16];
    let yes = f(0, &p, &v);
    println!("{}", if yes { "Yes" } else { "No" });
}
