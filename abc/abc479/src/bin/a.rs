use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let result = a.iter().step_by(2).sum::<usize>();
    println!("{result}");
}
