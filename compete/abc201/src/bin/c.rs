use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let f = |i: usize| -> bool {
        let mut a = [false; 10];
        for j in 0..=3 {
            let j = (i / 10usize.pow(j)) % 10;
            a[j] = true;
        }
        (0..10).all(|j| (a[j] && s[j] != 'x') || (!a[j] && s[j] != 'o'))
    };

    let result = (0..10000).filter(|&i| f(i)).count();
    println!("{}", result);
}
