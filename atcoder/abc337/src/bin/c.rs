use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let i0 = a.iter().position(|&x| x == -1).unwrap(); // 先頭

    let mut a0 = vec![0; n]; // 次の人
    for (i, &x) in a.iter().enumerate() {
        if x != -1 {
            a0[(x - 1) as usize] = i;
        }
    }

    let mut v = Vec::with_capacity(n);
    v.push(i0);
    while v.len() < n {
        let i = *v.last().unwrap();
        v.push(a0[i]);
    }

    let result = v.iter().map(|&x| x + 1).join(" ");
    println!("{result}");
}
