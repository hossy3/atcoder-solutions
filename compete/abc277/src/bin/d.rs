use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }

    a.sort();
    let mut v = Vec::new();
    for i in 0..n {
        if i == 0 || a[i] - a[i - 1] > 1 {
            v.push((i, a[i]));
        } else {
            let len = v.len();
            v[len - 1].1 += a[i];
        }
    }
    if v.len() > 1 && a[0] == 0 && a[n - 1] == m - 1 {
        let len = v.len();
        v[len - 1].1 += v[0].1;
    }

    let all: usize = a.iter().sum();
    let max = v.iter().map(|x| x.1).max().unwrap();
    println!("{}", all - max);
}
