use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }
    let mut a0 = [0usize; 46];
    let mut b0 = [0usize; 46];
    let mut c0 = [0usize; 46];
    for a in a {
        a0[a % 46] += 1;
    }
    for b in b {
        b0[b % 46] += 1;
    }
    for c in c {
        c0[c % 46] += 1;
    }
    let mut count = 0usize;
    for i in 0..46 {
        for j in 0..46 {
            let k = (46 * 2 - i - j) % 46;
            count += a0[i] * b0[j] * c0[k]
        }
    } 
    println!("{}", count);
}
