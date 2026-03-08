use proconio::input;

fn main() {
    input! {
        n: usize,
        mut v: [usize; n],
    }
    v.sort();
    let result = v.windows(2).map(|v| v[1] - v[0]).sum::<usize>();
    println!("{result}");
}
