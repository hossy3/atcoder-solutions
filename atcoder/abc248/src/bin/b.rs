use proconio::input;

fn main() {
    input! {
        mut a: usize,
        b: usize,
        k: usize,
    }
    let mut n = 0;
    while a < b {
        a *= k;
        n += 1;
    }
    println!("{}", n);
}
