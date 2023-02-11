use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let mut v: Vec<(usize, usize)> = vec![];
    for &i in &a {
        let j = v.len();
        if j > 0 && v[j - 1].1 == i {
            v[j - 1].1 = i + 1;
        } else {
            v.push((i, i + 1));
        }
    }

    let mut a = Vec::with_capacity(n);
    for i in 1..=n {
        a.push(i);
    }
    for (i, j) in v {
        for k in i..=j {
            a[k - 1] = j + i - k;
        }
    }
    let result = a.iter().join(" ");
    println!("{}", result);
}
