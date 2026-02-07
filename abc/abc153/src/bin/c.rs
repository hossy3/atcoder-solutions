use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [usize; n],
    }
    let result = if n > k {
        h.sort_unstable();
        h[..(n - k)].iter().sum::<usize>()
    } else {
        0
    };
    println!("{result}");
}
