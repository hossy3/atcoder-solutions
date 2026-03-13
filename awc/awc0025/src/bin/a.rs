use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    let result = a.iter().filter(|&ai| *ai < x).count();
    println!("{result}");
}
