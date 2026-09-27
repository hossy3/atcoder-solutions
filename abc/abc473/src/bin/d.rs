use itertools::Itertools;
use proconio::input;

fn f(n: usize, k: usize, k0: usize, v: &mut Vec<usize>) {
    if v.len() == n - 1 {
        if (k - k0) % (v.len() + 1) == 0 {
            let x = (k - k0) / (v.len() + 1);
            v.push(x);
            println!("{}", v.iter().join(" "));
            v.pop();
        }
        return;
    }

    for x in 0..=((k - k0) / (v.len() + 1)) {
        let k0 = k0 + x * (v.len() + 1);
        v.push(x);
        f(n, k, k0, v);
        v.pop();
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    f(n, k, 0, &mut vec![]);
}
