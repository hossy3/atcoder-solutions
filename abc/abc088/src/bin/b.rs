use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort_by(|a, b| b.cmp(a));

    let alice = a
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &x)| acc + if i % 2 == 0 { x } else { 0 });
    let bob = a
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &x)| acc + if i % 2 == 1 { x } else { 0 });
    let result = alice - bob;
    println!("{result}");
}
