use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
        b: [isize; n],
    }

    let result = (0..n).map(|i| a[i].abs_diff(b[i])).sum::<usize>();
    println!("{result}");
}
