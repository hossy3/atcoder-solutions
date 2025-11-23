use proconio::input;

fn f(x: i64, a: &Vec<i64>) -> i64 {
    for i in 0..=100 {
        let mut a = a.clone();
        a.push(i);
        a.sort();
        let x0: i64 = a[1..(a.len() - 1)].iter().sum();
        if x0 >= x {
            return i;
        }
    }
    return -1;
}

fn main() {
    input! {
        n: i64,
        x: i64,
        a: [i64; n - 1],
    }
    let result = f(x, &a);
    println!("{result}");
}
