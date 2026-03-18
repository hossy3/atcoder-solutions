use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [isize; n],
        a: [isize; n],
        b: [isize; n],
    }

    let sum_a = a.iter().sum::<isize>();
    let sum_b = b.iter().sum::<isize>();
    let add_a = (0..n).map(|i| p[i] - a[i]).max().unwrap();

    let result = sum_a + add_a - sum_b;
    println!("{result}");
}
