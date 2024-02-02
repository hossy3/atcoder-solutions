use ac_library::z_algorithm_arbitrary;
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn build_nums(s: &[char]) -> Vec<usize> {
    let mut v = Vec::with_capacity(s.len());
    for x in s {
        let i = match x {
            'R' => 0,
            'G' => 1,
            'B' => 2,
            _ => unreachable!(),
        };
        v.push(i);
    }
    v
}

fn f(s: &[usize], t: &[usize]) -> usize {
    f_by_z_algorithm(s, t)
}

fn f_by_z_algorithm(s: &[usize], t: &[usize]) -> usize {
    let n = s.len();

    let v: Vec<&usize> = s.iter().chain(t.iter()).collect();
    let z = z_algorithm_arbitrary(&v);
    let mut result = (0..n).filter(|&i| z[n + i] >= n - i).count();

    let v: Vec<&usize> = t.iter().chain(s.iter()).collect();
    let z = z_algorithm_arbitrary(&v);
    result += (1..n).filter(|&i| z[n + i] >= n - i).count();

    result
}

fn main() {
    input! {
        _: usize,
        s: Chars,
        t: Chars,
    }

    let s = build_nums(&s);
    let t = build_nums(&t);

    let result = f(&s, &t.iter().map(|&i| [0, 2, 1][i]).collect_vec())
        + f(&s, &t.iter().map(|&i| [1, 0, 2][i]).collect_vec())
        + f(&s, &t.iter().map(|&i| [2, 1, 0][i]).collect_vec());
    println!("{result}");
}
