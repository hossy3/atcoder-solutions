use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let result = (a + b).pow(2);
    println!("{result}");
}
