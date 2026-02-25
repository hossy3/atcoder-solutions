use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        e: [usize; n],
        c: [usize; m],
    }
    let result = *e.iter().min().unwrap() * c.iter().sum::<usize>();
    println!("{result}");
}
