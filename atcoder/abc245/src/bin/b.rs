use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v = [false; 2001];
    for i in a {
        v[i] = true;
    }
    for i in 0..=2000 {
        if !v[i] {
            println!("{}", i);
            return;
        }
    }
}
