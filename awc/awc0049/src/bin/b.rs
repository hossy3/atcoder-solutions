use proconio::input;

fn main() {
    input! {
        s: u128,
        t: u128,
        k: u32,
    }
    for i in 0..=k {
        let x = s * 2u128.pow(i);
        if x == t {
            println!("{i}");
            return;
        } else if x > t {
            break;
        }
    }
    println!("-1");
}
