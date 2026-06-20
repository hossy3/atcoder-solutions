use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut cur = a[..k].iter().sum::<usize>();
    let mut max = cur;
    let mut min = cur;
    for i in k..n {
        cur += a[i];
        cur -= a[i - k];
        max = max.max(cur);
        min = min.min(cur);
    }
    let result = max - min;
    println!("{result}");
}
