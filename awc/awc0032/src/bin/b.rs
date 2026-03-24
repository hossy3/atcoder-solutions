use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        a: [usize; n],
    }
    let result = a.iter().map(|&x| x.div_ceil(k + m)).sum::<usize>();
    println!("{result}");
}
