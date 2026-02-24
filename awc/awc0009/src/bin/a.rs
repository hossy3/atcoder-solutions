use proconio::input;

fn main() {
    input! {
        k: usize,
        m: usize,
        l: [usize; k],
    }
    let result = l.iter().sum::<usize>() % m;
    println!("{result}");
}
