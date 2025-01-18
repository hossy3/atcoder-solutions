use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut result = 0usize;
    for i in 0..n {
        let x = a[i];
        let len = a.len() - a.partition_point(|&y| y < x * 2);
        result += len;
    }
    println!("{result}");
}
