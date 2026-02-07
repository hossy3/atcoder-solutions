use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    }

    let mut cur: usize = p[0..k].iter().sum();
    let mut max = cur;
    for i in 0..(n - k) {
        cur -= p[i];
        cur += p[i + k];
        max = max.max(cur);
    }

    let result = (max + k) as f64 / 2.0;
    println!("{result}");
}
