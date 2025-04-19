use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u32,
    }
    const MAX: usize = 1_000_000_000_usize;
    let mut x = 0usize;
    for i in 0..=m {
        x += n.pow(i);
        if x > MAX {
            println!("inf");
            return;
        }
    }
    println!("{x}");
}
