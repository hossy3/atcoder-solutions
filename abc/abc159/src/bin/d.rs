use proconio::input;

fn f(x: isize) -> isize {
    x * (x - 1) / 2
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut v = vec![0isize; n + 1];
    for &x in &a {
        v[x] += 1;
    }
    let base = v.iter().map(|&x| f(x)).sum::<isize>();

    for &x in &a {
        let x = v[x];
        let result = base - f(x) + f(x - 1);
        println!("{result}");
    }
}
