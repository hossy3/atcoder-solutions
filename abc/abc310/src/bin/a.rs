use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        d: [usize; n],
    }
    let result = p.min(q + *d.iter().min().unwrap());
    println!("{}", result);
}
