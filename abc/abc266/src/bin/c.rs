use proconio::input;

fn f(a: &[(i32, i32)]) -> bool {
    for i in 0..4 {
        let x = a[i];
        let y = a[(i + 1) % 4];
        let z = a[(i + 2) % 4];
        let b = (y.0 - x.0, y.1 - x.1);
        let c = (z.0 - y.0, z.1 - y.1);
        let z = (b.1 * c.0) - (b.0 * c.1);
        if z > 0 {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        a: [(i32, i32); 4],
    }
    let yes = f(&a);
    println!("{}", if yes { "Yes" } else { "No" });
}
