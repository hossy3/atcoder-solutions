use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    let mut a = vec![0usize; n.len() + 1];
    for (i, &c) in n.iter().enumerate() {
        let b = 1usize << ((c as u8) - ('0' as u8));
        a[i + 1] = a[i] ^ b;
    }

    let mut m = HashMap::new();
    for &x in &a {
        if let Some(num) = m.get_mut(&x) {
            *num += 1;
        } else {
            m.insert(x, 1);
        }
    }

    let mut result = 0usize;
    for (_, &num) in &m {
        result += num * (num - 1) / 2;
    }
    println!("{}", result);
}
