use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut v = vec![0; 101];
    for &a in &a {
        for i in 0..=(a.min(100)) {
            v[i] += 1;
        }
    }

    for i in (0..=100).rev() {
        if v[i] >= i {
            println!("{i}");
            return;
        }
    }
    unreachable!();
}
