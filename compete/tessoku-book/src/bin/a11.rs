use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    if let Ok(i) = a.binary_search(&x) {
        println!("{}", i + 1);
    } else {
        panic!();
    }
}
