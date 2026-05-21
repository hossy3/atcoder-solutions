use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut fb: [(isize, isize); n],
    }

    fb.sort_unstable_by_key(|&(f, b)| b - f); // ひっくり返すとスコアが上がる方を奥に
    for i in (n - k)..n {
        fb[i] = (fb[i].1, fb[i].0);
    }

    let result = fb.iter().map(|&(f, _)| f).sum::<isize>();
    println!("{result}");
}
