use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        t: [usize; n],
    }

    let mut v = vec![0usize; n + 1];
    for (i, &x) in t.iter().enumerate() {
        if l <= x && x <= r {
            v[i + 1] = v[i] + 1;
        } else {
            v[i + 1] = 0;
        }
    }

    let result = *v.iter().max().unwrap();
    println!("{result}");
}
