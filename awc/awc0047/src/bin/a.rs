use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        t: [isize; n],
    }
    let result = t.windows(2).filter(|v| v[0].abs_diff(v[1]) >= k).count();
    println!("{result}");
}
