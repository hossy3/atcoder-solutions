use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    }

    let mut v = vec![1usize];
    for _ in 0..n {
        input! {
            row: [usize],
        }
        let v0 = v.clone();
        v.clear();

        for &i in &v0 {
            for &j in &row {
                if x / i >= j {
                    v.push(i * j);
                }
            }
        }
    }

    let result = v.iter().filter(|&&x0| x0 == x).count();
    println!("{}", result);
}
