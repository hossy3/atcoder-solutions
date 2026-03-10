use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut v = vec![];
    for _ in 0..n {
        input! {
            c: usize,
            m: usize,
            p: [usize; m],
        }
        v.push(p.iter().sum::<usize>().saturating_sub(c));
    }

    v.sort_unstable();
    v.reverse();
    let result = v[0..k].iter().sum::<usize>();

    println!("{result}");
}
