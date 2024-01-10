use proconio::input;

fn f(a: &[usize], b: usize) -> usize {
    match a.binary_search(&b) {
        Ok(_) => 0,
        Err(i) => {
            if i == 0 {
                a[0] - b
            } else if i == a.len() {
                b - a[i - 1]
            } else {
                (b - a[i - 1]).min(a[i] - b)
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
        b: [usize; q],
    };
    a.sort();
    for b in b {
        let result = f(&a, b);
        println!("{result}");
    }
}
