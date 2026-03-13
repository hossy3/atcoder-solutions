use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        r: usize,
        t: [usize; n],
    }

    let result = t.iter().sum::<usize>() + m * r;
    println!("{result}");
}
