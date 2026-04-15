use ac_library::z_algorithm_arbitrary;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let v: Vec<_> = b.iter().chain(a.iter()).collect();
    let lcp = z_algorithm_arbitrary(&v);
    for i in 0..n {
        if lcp[i + m] >= m {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
