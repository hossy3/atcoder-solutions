use proconio::input;

fn f(v: &[(i64, i64)]) -> i64 {
    let mut m = [vec![], vec![]];
    for &(x, y) in v {
        m[0].push(y + x);
        m[1].push(y - x);
    }
    m[0].sort();
    m[1].sort();

    let mut result = 0;
    for v in m {
        let mut sum = 0;
        for (i, &x) in v.iter().enumerate() {
            result += x * i as i64 - sum;
            sum += x;
        }
    }

    result / 2
}

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut v0 = vec![]; // even
    let mut v1 = vec![]; // odd
    for &(x, y) in &xy {
        if (y - x) % 2 == 0 {
            v0.push((x, y));
        } else {
            v1.push((x, y));
        }
    }
    v0.sort();
    v1.sort();
    let result = f(&v0) + f(&v1);
    println!("{result}");
}
