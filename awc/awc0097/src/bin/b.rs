use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        t: [usize; n],
    }

    let mut cur = t[..k].iter().sum::<usize>();
    let mut result = (cur * 1000) / k;
    for i in k..n {
        cur -= t[i - k];
        cur += t[i];
        result = result.max((cur * 1000) / k);
    }
    println!("{result}");
}
