use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    }

    let mut v = vec![];
    for &(a, b) in &ab {
        v.push(a - b); // どれだけお得？
    }
    v.sort();
    let result = ab.iter().map(|&(a, _)| a).sum::<usize>() - v[(n - k)..].iter().sum::<usize>();
    println!("{result}");
}
