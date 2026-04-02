use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut d: [usize; n],
    }
    d.sort_unstable();
    let result = d[k..].iter().sum::<usize>();
    println!("{result}");
}
