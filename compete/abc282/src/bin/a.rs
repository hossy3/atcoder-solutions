use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    for i in 0..n {
        let c = ('A' as u8 + i as u8) as char;
        print!("{}", c);
    }
    println!();
}
