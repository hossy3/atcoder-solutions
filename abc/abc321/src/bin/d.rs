use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: i64,
        a: [i64; n],
        mut b: [i64; m],
    }

    b.sort();
    let mut b_acc = vec![0; m + 1];
    for (i, &b) in b.iter().enumerate() {
        b_acc[i + 1] = b_acc[i] + b;
    }

    let mut result = 0;
    for &a in &a {
        let i = b.lower_bound(&(p - a));
        result += (a * i as i64 + b_acc[i]) + ((m - i) as i64) * p;
    }
    println!("{result}");
}
