use proconio::input;

fn f(x: usize, y: usize, k: u32, v: &mut Vec<usize>) {
    let z = 1 << k;
    if x < z {
        return;
    }
    f(x, y, k + 1, v);
    if x & z == z {
        v.push(y + z);
        f(x, y + z, k + 1, v);
    }
}

fn main() {
    input! {
        x: usize
    }
    let mut v = vec![0];
    f(x, 0, 0, &mut v);
    v.sort();
    for &x in &v {
        println!("{}", x);
    }
}
