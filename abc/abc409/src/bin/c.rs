use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        d: [usize; n - 1],
    }

    if l % 3 != 0 {
        println!("0");
        return;
    }

    let mut v = vec![0usize; l];
    let mut cur = 0;
    v[cur] += 1;
    for &d in &d {
        cur = (cur + d) % l;
        v[cur] += 1;
    }

    let mut result = 0;
    for i in 0..(l / 3) {
        result += v[i] * v[l / 3 + i] * v[l / 3 * 2 + i];
    }
    println!("{result}");
}
