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
    let mut ab = vec![0usize; 46];
    for (i, &a0) in a.iter().enumerate() {
        for (j, &b0) in b.iter().enumerate() {
            let k = (i + j) % 46;
            ab[k] += a0 * b0;
        }
    }
    let mut count = 0usize;
    for (i, &ab0) in ab.iter().enumerate() {
        for (j, &c0) in c.iter().enumerate() {
            if (i + j) % 46 == 0 {
                count += ab0 * c0;
            }
        }
    }
    println!("{count}");
}
