use proconio::input;

fn f(a: &[usize]) -> [usize; 46] {
    let mut a0 = [0usize; 46];
    for a in a {
        a0[a % 46] += 1;
    }
    a0
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }
    let a = f(&a);
    let b = f(&b);
    let c = f(&c);
    let mut count = 0usize;
    for (i, &a0) in a.iter().enumerate() {
        for (j, &b0) in b.iter().enumerate() {
            let c0 = c[(46 * 2 - i - j) % 46];
            count += a0 * b0 * c0;
        }
    }
    println!("{count}");
}
