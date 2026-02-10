use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    for (i, &x) in a.iter().enumerate() {
        if x == k {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
