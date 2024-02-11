use proconio::input;

const N: usize = 100_000;

fn f(x: usize) -> usize {
    let y = (x % 10) + ((x / 10) % 10) + ((x / 100) % 10) + ((x / 1000) % 10) + ((x / 10000) % 10);
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
