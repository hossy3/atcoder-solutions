use proconio::input;

fn main() {
    input! {
        x: usize,
    }
    let mut x0 = 100usize;
    for result in 1.. {
        x0 = x0 + x0 / 100; // * 101 / 100 すると overflow
        if x0 >= x {
            println!("{result}");
            break;
        }
    }
}
