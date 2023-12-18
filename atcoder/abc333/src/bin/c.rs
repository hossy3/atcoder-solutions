use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }

    const N: usize = 12;
    let mut a = vec![0usize; N];
    a[0] = 1;
    for i in 0..(N - 1) {
        a[i + 1] = a[i] * 10 + 1;
    }

    let mut v = vec![];
    for i in 0..N {
        for j in i..N {
            for k in j..N {
                v.push(a[i] + a[j] + a[k]);
            }
        }
    }
    v.sort();

    let result = v[n - 1];
    println!("{result}");
}
