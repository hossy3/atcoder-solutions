use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v = vec![0; n * 2 + 2];
    for i in 1..=n {
        let j = a[i - 1];
        v[i * 2] = v[j] + 1;
        v[i * 2 + 1] = v[j] + 1;
    }
    for &x in &v[1..] {
        println!("{}", x);
    }
}
