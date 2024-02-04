use proconio::input;

fn f(a: &[usize], b: usize) -> usize {
    match a.binary_search(&b) {
        Ok(_) => 0,
        Err(i) => {
            let x0 = a[i.saturating_sub(1)];
            let x1 = a[i.min(a.len() - 1)];
            (b.abs_diff(x0)).min(b.abs_diff(x1))
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
