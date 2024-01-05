use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }
    let mut v = vec![a[0], b[0]];
    for i in 1..n {
        let mut v0 = vec![];
        if v.iter().any(|x| (x - a[i]).abs() <= k) {
            v0.push(a[i]);
        }
        if v.iter().any(|x| (x - b[i]).abs() <= k) {
            v0.push(b[i]);
        }
        if v0.len() == 0 {
            println!("{}", "No");
            return;
        }
        v = v0;
    }
    println!("{}", "Yes");
}
