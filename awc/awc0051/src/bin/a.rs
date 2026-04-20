use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        p: [usize; n],
        l: [usize; m],
        t: [Usize1; k],
    }

    let l_max = t.iter().map(|&t| l[t]).max().unwrap();
    let result = p.iter().filter(|&&p| p <= l_max).sum::<usize>();
    println!("{result}");
}
