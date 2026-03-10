use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u8; n],
    }

    let mut v = vec![false; n]; // ひとつ前と異なる電球 = 操作すべきものは true
    v[0] = a[0] == 1;
    for i in 1..n {
        v[i] = a[i] != a[i - 1];
    }

    let mut result = 0usize;
    for i in 0..(n - k) {
        if v[i] {
            v[i] = false;
            v[i + k] = !v[i + k];
            result += 1;
        }
    }
    if v[n - k] {
        v[n - k] = false;
        result += 1;
    }

    if v.iter().all(|&b| !b) {
        println!("{result}");
    } else {
        println!("-1");
    }
}
