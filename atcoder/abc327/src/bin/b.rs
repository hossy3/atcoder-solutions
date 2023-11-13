use proconio::input;

fn main() {
    input! {
        b: u128,
    }
    for i in 1u128..=20u128 {
        let n = i.pow(i as u32);
        if n == b {
            println!("{i}");
            return;
        }
    }
    println!("-1");
}
