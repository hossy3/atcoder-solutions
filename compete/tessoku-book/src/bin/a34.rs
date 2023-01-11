use proconio::input;

// grundy number

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        a: [usize; n],
    }

    let max = *a.iter().max().unwrap();
    let mut grandy = vec![0; max + 1];
    for i in x..y {
        grandy[i] = 1 - grandy[i - x];
    }
    for i in y..=max {
        let mut v = [false; 3];
        v[grandy[i - y]] = true;
        v[grandy[i - x]] = true;
        grandy[i] = if !v[0] {
            0
        } else if !v[1] {
            1
        } else {
            2
        }
    }

    let result = a.iter().fold(0, |acc, a| acc ^ grandy[*a]);
    let first = result != 0;
    println!("{}", if first { "First" } else { "Second" });
}
