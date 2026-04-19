use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    a.sort_unstable();
    a.reverse();

    let mut result = 0;
    for (i, &x) in a.iter().enumerate() {
        result += a[(i + 1)..].partition_point(|&x0| x + x0 >= k);
    }
    println!("{result}");
}
