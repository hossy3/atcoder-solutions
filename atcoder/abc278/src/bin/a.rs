use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }
    for i in 0..n {
        let m = if (i + k) < n { a[i + k] } else { 0 };
        if i == n - 1 {
            println!("{}", m);
        } else {
            print!("{} ", m);
        }
    }
}
