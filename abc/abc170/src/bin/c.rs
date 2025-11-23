use proconio::input;

fn f(x: i64, p: &[i64]) -> i64 {
    for i in 0..=50 {
        if !p.contains(&(x - i)) {
            return x - i;
        }
        if !p.contains(&(x + i)) {
            return x + i;
        }
    }
    unreachable!();
}

fn main() {
    input! {
        x: i64,
        n: i64,
        p: [i64; n],
    }
    let result = f(x, &p);
    println!("{result}");
}
