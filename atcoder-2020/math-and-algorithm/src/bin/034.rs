use proconio::input;

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }
    let mut result = std::usize::MAX;
    for i in 0..n {
        for j in (i + 1)..n {
            let len2 = abs_diff(xy[i].0, xy[j].0).pow(2) + abs_diff(xy[i].1, xy[j].1).pow(2);
            result = result.min(len2);
        }
    }
    let result = (result as f64).sqrt();
    println!("{}", result);
}
