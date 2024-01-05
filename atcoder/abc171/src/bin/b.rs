use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut p: [usize; n],
    }
    p.sort();
    let result: usize = p[0..k].iter().sum();
    println!("{result}");
}
