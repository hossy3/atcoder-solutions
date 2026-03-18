use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        e: usize,
        mut p: [usize; n],
    }

    p.sort();
    let mut sum = 0;
    for (i, &p) in p.iter().enumerate() {
        sum += p * t;
        if sum > e {
            println!("{i}");
            return;
        }
    }

    println!("{n}");
}
