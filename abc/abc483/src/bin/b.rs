use proconio::input;

fn main() {
    input! {
        n: isize,
        q: usize,
        k: [usize; q],
    }

    let mut v: Vec<_> = (1..=n).into_iter().collect();
    for k in k {
        let v0 = v.clone();
        for i in 0..k {
            v[i] = -v0[k - i - 1];
        }
    }

    for x in v {
        println!("{x}");
    }
}
