use proconio::input;

fn main() {
    input! {
        n: usize,
        tlr: [(usize, usize, usize); n],
    }
    let mut lr = Vec::with_capacity(n);
    let mut result = 0;
    for &(t, l, r) in &tlr {
        let (l, r) = match t {
            1 => (l * 2, r * 2 + 1),
            2 => (l * 2, r * 2),
            3 => (l * 2 + 1, r * 2 + 1),
            4 => (l * 2 + 1, r * 2),
            _ => panic!(),
        };
        for &(l0, r0) in &lr {
            if (l <= l0 && l0 < r) || (l0 <= l && l < r0) {
                result += 1;
            }
        }
        lr.push((l, r));
    }
    println!("{}", result);
}
