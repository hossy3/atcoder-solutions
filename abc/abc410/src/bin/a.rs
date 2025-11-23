use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        k: usize,
    }
    let result = a.iter().filter(|&&x| k <= x).count();
    println!("{result}");
}
