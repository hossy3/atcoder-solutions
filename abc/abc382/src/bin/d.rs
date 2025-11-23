use itertools::Itertools;
use proconio::input;

fn f(n: usize, m: usize, len: usize, last: usize) -> usize {
    if n == len {
        1
    } else {
        let rest = m - (last + (n - len) * 10);
        let mut result = 0;
        for i in 0..=rest {
            result += f(n, m, len + 1, last + 10 + i);
        }
        result
    }
}

fn f_out(n: usize, m: usize, v: &mut Vec<usize>) {
    if v.len() == n {
        let result = v.iter().join(" ");
        println!("{result}");
    } else {
        let last = *v.last().unwrap();
        let len = v.len();
        let rest = m - (last + (n - len) * 10);
        for i in 0..=rest {
            v.push(last + 10 + i);
            f_out(n, m, v);
            v.pop();
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut len = 0;
    for i in 1..=(m - (n - 1) * 10) {
        len += f(n, m, 1, i);
    }
    println!("{len}");

    for i in 1..=(m - (n - 1) * 10) {
        let mut v = vec![i];
        len += f(n, m, 1, i);
        f_out(n, m, &mut v);
    }
}
