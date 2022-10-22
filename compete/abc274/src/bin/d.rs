use proconio::input;

fn f(a: &[usize], s: i32, g: i32) -> bool {
    const N: usize = 20001;
    let mut v = [false; N];
    v[(s + 10000) as usize] = true;
    for &i in a {
        let mut v0 = [false; N];
        for j in 0..v0.len() {
            if v[j] {
                if j >= i {
                    v0[j - i] = true;
                }
                if j + i < N {
                    v0[j + i] = true;
                }
            }
        }
        v = v0;
    }
    v[(g + 10000) as usize]
}

fn main() {
    input! {
        n: usize,
        x: i32,
        y: i32,
        a: [usize; n],
    }
    let mut v0 = vec![0; (n / 2) + (n % 2)];
    let mut v1 = vec![0; n / 2];
    for i in 0..((n / 2) + (n % 2)) {
        v0[i] = a[i * 2];
    }
    for i in 0..(n / 2) {
        v1[i] = a[i * 2 + 1];
    }
    let yes = f(&v0[1..], a[0] as i32, x) && f(&v1, 0, y);
    println!("{}", if yes { "Yes" } else { "No" });
}
