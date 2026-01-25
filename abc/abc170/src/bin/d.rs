use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();
    let max = *a.iter().max().unwrap();

    let mut v = vec![false; max + 1];
    for &x in &a {
        v[x] = true;
    }
    for &x in &a {
        if v[x] {
            for x in ((x + x)..=max).step_by(x) {
                v[x] = false; // 約数が別の場所にある
            }
        }
    }
    for x in a.windows(2).filter(|&v| v[0] == v[1]) {
        v[x[0]] = false; // 同じ値が別の場所にある
    }
    let result = v.iter().filter(|&&b| b).count();
    println!("{result}");
}
