use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut result = std::usize::MAX;
    for (i, &(a, _)) in ab.iter().enumerate() {
        if a > result {
            continue;
        }
        for (j, &(_, b)) in ab.iter().enumerate() {
            if i == j {
                continue;
            }
            result = result.min(a.max(b));
        }
    }
    for &(a, b) in &ab {
        result = result.min(a + b);
    }
    println!("{}", result);
}
