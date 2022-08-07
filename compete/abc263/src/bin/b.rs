use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [i64; n - 1],
    }
    let mut x = 0;
    let mut i = (n - 2) as i64;
    while i >= 0 {
        i = p[i as usize] - 2;
        x += 1;
    }
    println!("{}", x);
}
