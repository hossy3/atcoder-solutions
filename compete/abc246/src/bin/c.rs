use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        a: [usize; n],
    }
    let mut v = Vec::with_capacity(n);
    for &y in &a {
        let y = y % x;
        if y > 0 {
            v.push(y);
        }
    }

    let a_sum: usize = a.iter().sum();
    let v_sum: usize = v.iter().sum();
    let k0 = (a_sum - v_sum) / x;
    let sum = if k0 >= k {
        a_sum - k * x
    } else if k0 + v.len() > k {
        v.sort();
        v[..(v.len() - (k - k0))].iter().sum()
    } else {
        0
    };
    println!("{}", sum);
}
