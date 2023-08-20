use proconio::input;
use superslice::Ext;

fn f(k: usize, a: &[usize]) -> Vec<Vec<usize>> {
    let n = a.len();
    let mut v = vec![vec![0; 0]; k + 1];
    for i in 0..((1 << n) as usize) {
        let m = i.count_ones() as usize;
        if m > k {
            continue;
        }
        let mut x = 0;
        for j in 0..n {
            if ((1 << j) & i) > 0 {
                x += a[j];
            }
        }
        v[m].push(x);
    }
    for i in 1..=k {
        v[i].sort();
    }
    v
}

fn main() {
    input! {
        n: usize,
        k: usize,
        p: usize,
        mut a: [usize; n],
    }
    let v0 = f(k, &a[..(n / 2)]);
    let v1 = f(k, &a[(n / 2)..]);
    let mut count = 0;
    for i in 0..=k {
        for &x in &v1[i] {
            if x > p {
                continue;
            }
            let y = p - x;
            let j = v0[k - i].upper_bound(&y);
            count += j;
        }
    }
    println!("{}", count);
}
