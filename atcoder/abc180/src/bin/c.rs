use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut v = vec![];
    let n0 = (n as f64).sqrt().floor() as usize;
    for i in 1..=n0 {
        if n % i == 0 {
            v.push(i);
            if n / i != i {
                v.push(n / i);
            }
        }
    }
    v.sort();
    for x in v {
        println!("{x}");
    }
}
