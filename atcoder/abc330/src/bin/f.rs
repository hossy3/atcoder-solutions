use itertools::Itertools;
use proconio::input;

fn f_pos(a: &[i64], l: i64) -> i64 {
    let mut a0 = a.to_vec();
    let mut a1 = a0.iter().map(|x| x - l).collect_vec();
    a0.append(&mut a1);
    a0.sort();
    a0[a.len()]
}

fn f(a: &[i64], l: i64) -> i64 {
    let pos = f_pos(&a, l);
    let mut cost = 0;
    for &x in a {
        if x < pos {
            cost += pos - x;
        } else if x > pos + l {
            cost += x - (pos + l);
        }
    }
    cost
}

fn main() {
    input! {
        n: usize,
        k: i64,
        xy: [(i64, i64); n],
    }

    let mut xs = Vec::with_capacity(n);
    let mut ys = Vec::with_capacity(n);
    for &(x, y) in &xy {
        xs.push(x);
        ys.push(y);
    }
    xs.sort();
    ys.sort();

    let mut l = 0;
    let mut r = 10_i64.pow(9);
    while l != r {
        let m = (l + r) / 2;
        if f(&xs, m) + f(&ys, m) <= k {
            r = m;
        } else {
            l = m + 1;
        }
    }
    println!("{l}");
}
