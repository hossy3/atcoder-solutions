use proconio::input;

fn f(v: &mut [usize]) -> usize {
    let mut count = 0;
    for i in (0..v.len()).rev() {
        if v[i] != i {
            let j = v.iter().position(|&x| x == i).unwrap();
            for k in j..i {
                v.swap(k, k + 1);
                count += 1;
            }
        }
    }
    count
}

fn main() {
    input! {
        n: usize,
        p: [[usize; n]; n],
    }

    let mut v0 = Vec::with_capacity(n);
    for i in 0..n {
        v0.push((0..n).map(|j| p[i][j]).max().unwrap() - 1);
    }

    let mut v1 = Vec::with_capacity(n);
    for j in 0..n {
        v1.push((0..n).map(|i| p[i][j]).max().unwrap() - 1);
    }

    let count = f(&mut v0) + f(&mut v1);
    println!("{}", count);
}
