use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        sc: [(usize, usize); m],
    }

    let pow10 = [1, 10, 100, 1000];
    let l = if n == 1 { 0 } else { pow10[n - 1] };
    let r = pow10[n];
    for x in l..r {
        if sc.iter().all(|&(s, c)| (x / pow10[n - s]) % 10 == c) {
            println!("{x}");
            return;
        }
    }
    println!("-1");
}
