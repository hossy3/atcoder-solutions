use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    a.sort();
    let mut acc = a.clone();
    for i in 0..(n - 1) {
        acc[i + 1] += acc[i];
    }
    let mut result = 0;
    for (i, (x, y)) in std::iter::zip(a, acc).into_iter().enumerate() {
        result += x * (i + 1) as i64 - y;
    }
    println!("{result}");
}
