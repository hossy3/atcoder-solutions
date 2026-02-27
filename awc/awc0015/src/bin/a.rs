use proconio::input;

fn main() {
    input! {
        a: [isize; 7],
        b: [isize; 7],
    }
    let result = a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum::<isize>();
    println!("{result}");
}
