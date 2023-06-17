use proconio::input;

fn main() {
    input! {
        a: [usize; 64],
    }
    let result: usize = (0..64).map(|i| a[i] * (1 << i as u8)).sum();
    println!("{}", result);
}
