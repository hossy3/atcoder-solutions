use proconio::input;

fn f(x: isize) -> (isize, isize) {
    let mut a = -200_isize; // 200 ** 5 - 199 ** 5 > 10 ** 9
    let mut b = -200_isize;
    while a <= 200 {
        let x0 = a.pow(5) - b.pow(5);
        if x0 == x {
            return (a, b);
        } else if x0 < x {
            a += 1;
        } else {
            b += 1;
        }
    }
    unreachable!();
}

fn main() {
    input! {
        x: isize,
    }
    let (a, b) = f(x);
    println!("{a} {b}");
}
