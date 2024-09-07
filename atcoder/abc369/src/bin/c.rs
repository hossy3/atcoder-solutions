use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut diff = vec![0i64; n - 1];
    for i in 0..(n - 1) {
        diff[i] = a[i + 1] - a[i];
    }

    let mut v: Vec<(i64, usize)> = vec![];
    for i in 0..(n - 1) {
        let j = v.len();
        if j == 0 || v[j - 1].0 != diff[i] {
            v.push((diff[i], 1));
        } else {
            v[j - 1].1 += 1;
        }
    }

    let mut result = 0;
    for &(_, x) in &v {
        result += (x + 2) * (x + 1) / 2;
    }
    result -= v.len() - 1;
    println!("{result}");
}
