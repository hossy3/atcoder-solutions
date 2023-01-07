use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    println!("{}", n);
    for i in 0..n {
        let j = (i + 1) % n;
        println!("{} {}", i + 1, j + 1);
    }
}
