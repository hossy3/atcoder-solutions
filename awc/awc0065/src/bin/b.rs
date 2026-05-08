use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    let mut sum = 0;
    for (i, &a) in a.iter().enumerate() {
        sum += a;
        if sum >= x {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
