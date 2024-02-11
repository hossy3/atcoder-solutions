use proconio::input;

const N: usize = 100_000;

fn f(x: usize) -> usize {
    let y: usize = (0..6).map(|k| (x / 10_usize.pow(k)) % 10).sum();
    (x + y) % N
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut v = vec![usize::MAX; N];
    let mut count = 0;
    let mut x = n;
    v[x] = 0;
    while count < k {
        x = f(x);
        count += 1;
        if v[x] < usize::MAX && (k - count) % (count - v[x]) == 0 {
            break;
        }
        v[x] = count;
    }
    println!("{x}");
}
