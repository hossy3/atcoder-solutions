use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    a.sort_unstable();
    for i in (n - k)..n {
        a[i] /= 2;
    }
    let result = a.iter().sum::<usize>();
    println!("{result}");
}
