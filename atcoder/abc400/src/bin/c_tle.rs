use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut result = 0usize;
    for i in 1..=n {
        if i % 2 != 0 {
            continue;
        }
        let x = (i as f64).sqrt() as usize;
        if x.pow(2) == i || (x + 1).pow(2) == i {
            result += 1;
            continue;
        }

        let i = i / 2;
        let x = (i as f64).sqrt() as usize;
        if x.pow(2) == i || (x + 1).pow(2) == i {
            result += 1;
            continue;
        }
    }

    println!("{result}");
}
