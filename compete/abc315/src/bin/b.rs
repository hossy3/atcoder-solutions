use proconio::input;

fn main() {
    input! {
        m: usize,
        d: [usize; m],
    }

    let mid = d.iter().sum::<usize>() / 2;
    let mut sum = 0;
    for (i, &x) in d.iter().enumerate() {
        if sum + x > mid {
            let month = i + 1;
            let day = mid - sum + 1;
            println!("{month} {day}");
            return;
        }
        sum += x;
    }
}
