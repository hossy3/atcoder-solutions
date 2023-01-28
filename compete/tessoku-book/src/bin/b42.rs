use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }

    let mut t = (0, 0, 0, 0); // ++, +-, -+, --
    for &(a, b) in &ab {
        let x = a + b;
        if x > 0 {
            t.0 += x;
        } else {
            t.3 -= x;
        }

        let x = a - b;
        if x > 0 {
            t.1 += x;
        } else {
            t.2 -= x;
        }
    }
    let result = t.0.max(t.1).max(t.2).max(t.3);
    println!("{}", result);
}
