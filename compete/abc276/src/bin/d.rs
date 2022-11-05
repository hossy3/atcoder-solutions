use proconio::input;

fn f(mut x: usize) -> (usize, usize, usize) {
    let mut i = 0;
    while x % 2 == 0 {
        x /= 2;
        i += 1;
    }
    let mut j = 0;
    while x % 3 == 0 {
        x /= 3;
        j += 1;
    }
    (x, i, j)
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut k = 0;
    let mut vi = Vec::new();
    let mut vj = Vec::new();
    for &x in &a {
        let (k0, i, j) = f(x);
        if k == 0 {
            k = k0
        } else if k != k0 {
            println!("-1");
            return;
        }
        vi.push(i);
        vj.push(j);
    }
    let i_min = vi.iter().min().unwrap();
    let j_min = vj.iter().min().unwrap();
    let i_sum = vi.iter().fold(0, |acc, x| acc + x - i_min);
    let j_sum = vj.iter().fold(0, |acc, x| acc + x - j_min);
    println!("{}", i_sum + j_sum);
}
