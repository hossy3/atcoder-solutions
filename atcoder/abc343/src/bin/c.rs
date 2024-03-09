use proconio::input;

fn f(mut i: usize) -> bool {
    let mut v = vec![];
    while i > 0 {
        v.push(i % 10);
        i /= 10;
    }
    let v0 = v.clone();
    v.reverse();
    v == v0
}

fn main() {
    input! {
        n: usize,
    }

    let mut k = (n as f64).powf(1.0 / 3.0) as usize;
    if (k + 1).pow(3) <= n {
        k += 1;
    }

    for i in (1..=k).rev() {
        let result = i.pow(3);
        if f(result) {
            println!("{result}");
            break;
        }
    }
}
