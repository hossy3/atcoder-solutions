use proconio::input;

fn main() {
    input! {
        a: [usize],
    }

    let mut v = vec![0i64; *a.iter().max().unwrap()];
    for &a in &a {
        v[a - 1] += 1;
    }

    let count = v.iter().fold(0, |acc, x| acc + x * (x - 1) * (x - 2) / 6);
    println!("{}", count);
}
