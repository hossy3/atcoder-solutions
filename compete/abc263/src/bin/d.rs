use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i64,
        r: i64,
        a: [i64; n],
    }

    let mut a_l = vec![0i64; n + 1];
    for i in 0..n {
        a_l[i + 1] = (l * (i as i64 + 1)).min(a_l[i] + a[i]);
    }
    let mut a_r = vec![0i64; n + 1];
    for i in 0..n {
        let j = n - i - 1;
        a_r[j] = (r * (i as i64 + 1)).min(a_r[j + 1] + a[j]);
    }
    let mut score = 1 << 60;
    for i in 0..=n {
        score = score.min(a_l[i] + a_r[i]);
    }
    println!("{}", score);
}
