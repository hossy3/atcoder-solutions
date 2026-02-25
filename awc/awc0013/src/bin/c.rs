use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: [usize; n],
    }
    let x = s[k - 1];
    s.sort_unstable();
    let result = s.partition_point(|&x0| x0 < x);
    println!("{result}");
}
