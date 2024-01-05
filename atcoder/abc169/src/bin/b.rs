use proconio::input;

fn f(a: &[i128]) -> i128 {
    if a.contains(&0) {
        return 0;
    }

    const MAX: i128 = 10_i128.pow(18);
    let mut acc = 1;
    for x in a {
        acc *= x;
        if acc > MAX {
            return -1;
        }
    }

    acc
}

fn main() {
    input! {
        n: usize,
        a: [i128; n],
    }
    let result = f(&a);
    println!("{result}");
}
