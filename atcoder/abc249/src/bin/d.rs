use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let max = a.iter().max().unwrap();
    let mut counts = vec![0usize; max + 1];
    for i in 0..n {
        counts[a[i]] += 1;
    }

    let mut count = 0;
    for i in 1..=*max {
        if counts[i] == 0 {
            continue;
        }
        let j_max = max / i;
        for j in 1..=j_max {
            let k = i * j;
            count += counts[i] * counts[j] * counts[k];
        }
    }
    println!("{}", count);
}
