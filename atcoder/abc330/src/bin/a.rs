use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        a: [usize; n],
    }
    let result = a.iter().filter(|&&x| x >= l).count();
    println!("{result}");
}
