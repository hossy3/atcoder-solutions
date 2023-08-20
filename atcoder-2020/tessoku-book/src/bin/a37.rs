use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: usize,
        a: [usize; n],
        c: [usize; m],
    }

    let result = a.iter().sum::<usize>() * c.len()
        + b * a.len() * c.len()
        + c.iter().sum::<usize>() * a.len();
    println!("{}", result);
}
