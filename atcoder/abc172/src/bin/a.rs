use proconio::input;

fn main() {
    input! {
        a: usize,
    }
    let result = a + a * a + a * a * a;
    println!("{result}");
}
