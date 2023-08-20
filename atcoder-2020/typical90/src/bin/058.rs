use proconio::input;

fn f(x: usize) -> usize {
    let y = (x % 10) + ((x / 10) % 10) + ((x / 100) % 10) + ((x / 1000) % 10) + ((x / 10000) % 10);
    (x + y) % 100_000
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    const MAX: usize = 1 << 60;
    let mut v = vec![MAX; 100_000];
    let mut count = 0;
    let mut x = n;
    v[x] = 0;
    while count < k {
        x = f(x);
        count += 1;
        if v[x] < MAX && (k - count) % (count - v[x]) == 0 {
            break;
        }
        v[x] = count;
    }
    println!("{}", x);
}
