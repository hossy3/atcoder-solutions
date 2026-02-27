use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    }

    if let Some(i) = p.iter().position(|&x| x >= k) {
        println!("{}", i + 1);
    } else {
        println!("-1");
    }
}
