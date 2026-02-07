use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    for i in 1..=1000 {
        if i * 8 / 100 == a && i / 10 == b {
            println!("{i}");
            return;
        }
    }
    println!("-1");
}
