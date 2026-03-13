use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: usize,
        a: [usize; n],
    }
    for a in a {
        if a < x {
            x = a;
            println!("1");
        } else {
            println!("0");
        }
    }
}
