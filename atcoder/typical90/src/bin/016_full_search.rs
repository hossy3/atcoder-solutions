use proconio::input;

const M: usize = 9999;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
    }

    let mut result = usize::MAX;
    for i in 0..=M {
        if i * a > n {
            continue;
        }
        for j in 0..=(M - i) {
            if i * a + b * j > n {
                continue;
            }
            let rest = n - (i * a + b * j);
            if rest % c != 0 {
                continue;
            }
            let k = rest / c;
            result = result.min(i + j + k)
        }
    }

    println!("{result}");
}
