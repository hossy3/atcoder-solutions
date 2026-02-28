use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        mut k: usize,
    }
    let x = a.min(k);
    a -= x;
    k -= x;

    let x = b.min(k);
    b -= x;

    println!("{a} {b}");
}
