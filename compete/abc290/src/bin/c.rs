use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut v = vec![false; k + 1];
    for i in a {
        if i < k {
            v[i] = true;
        }
    }
    for i in 0..=k {
        if !v[i] {
            println!("{}", i);
            break;
        }
    }
}
