use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut x: [usize; n],
    }

    x.sort();
    let mut result = 0usize;
    for i in 0..n {
        let j = x.partition_point(|&x0| x0 <= x[i] + k);
        result = result.max(j - i);
    }
    println!("{result}");
}
