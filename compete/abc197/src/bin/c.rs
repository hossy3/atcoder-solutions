use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut result = std::usize::MAX;
    for i in 0..(1 << (n - 1)) {
        let mut acc_or = 0;
        let mut acc_xor = 0;
        for (j, &x) in a.iter().enumerate() {
            acc_or |= x;
            if (i & (1 << j) > 0) || (j == n - 1) {
                acc_xor ^= acc_or;
                acc_or = 0;
            }
        }
        result = result.min(acc_xor);
    }
    println!("{}", result);
}
