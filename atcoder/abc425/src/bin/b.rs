use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut v = vec![];
    for i in 1..=(n as isize) {
        let count = a.iter().filter(|&&j| i == j).count();
        match count {
            0 => {
                v.push(i);
            }
            1 => {}
            _ => {
                println!("No");
                return;
            }
        }
    }

    let mut results = a.clone();
    for i in (0..n).rev() {
        if results[i] == -1 {
            results[i] = v.pop().unwrap();
        }
    }
    println!("Yes");
    println!("{}", results.iter().join(" "));
}
