use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let mut v = a.clone();
    v.push(0);
    v.push(n + 1);
    v.sort();

    let v = v
        .iter()
        .tuple_windows()
        .filter(|&(&a, &b)| a + 1 != b)
        .map(|(&a, &b)| b - a - 1)
        .collect_vec();
    if v.len() == 0 {
        println!("0");
        return;
    }

    let min = *v.iter().min().unwrap();
    let result: usize = v
        .iter()
        .map(|&x| (x as f64 / min as f64).ceil() as usize)
        .sum();
    println!("{result}");
}
