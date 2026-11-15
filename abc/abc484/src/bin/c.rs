use proconio::input;

fn f(a: &[usize], b: &[usize]) -> bool {
    let n = a.len();

    // 全部一致していれば true
    if a == b {
        return true;
    }

    let mut i0 = usize::MAX; // 異なる値 左端
    for i in 0..n {
        if a[i] != b[i] {
            i0 = i;
            break;
        }
    }

    let mut i1 = usize::MAX; // 異なる値 右端
    for i in (0..n).rev() {
        if a[i] != b[i] {
            i1 = i;
            break;
        }
    }

    // a の i0 を i1 に移動して一致すれば OK
    if a[i0] == b[i1] && (i0..(i1 - 1)).all(|i| a[i + 1] == b[i]) {
        return true;
    }

    // a の i1 を i0 に移動して一致すれば OK
    if a[i1] == b[i0] && (i0..(i1 - 1)).all(|i| a[i] == b[i + 1]) {
        return true;
    }

    false
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let yes = f(&a, &b);
    println!("{}", if yes { "Yes" } else { "No" });
}
