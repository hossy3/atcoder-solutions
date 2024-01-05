use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        p: [usize; n],
    }
    for i in 0..n {
        if p[i] == x {
            println!("{}", i + 1);
            break;
        }
    }
}
