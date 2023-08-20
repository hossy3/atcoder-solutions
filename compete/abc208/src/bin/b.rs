use proconio::input;

fn main() {
    input! {
        mut p: usize,
    }
    let mut v = vec![1usize; 10];
    for i in 1..10 {
        v[i] = v[i - 1] * (i + 1);
    }
    let mut count = 0;
    for &x in v.iter().rev() {
        let i = (p / x).min(100);
        p -= i * x;
        count += i;
    }
    println!("{}", count);
}
