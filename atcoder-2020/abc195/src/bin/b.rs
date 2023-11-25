use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        w: usize,
    }

    let w = w * 1000;
    let mut min = std::usize::MAX;
    let mut max = std::usize::MIN;
    for i in 0..=(w / a) {
        if a * i <= w && w <= b * i {
            min = min.min(i);
            max = max.max(i);
        }
    }
    if min <= max {
        println!("{} {}", min, max);
    } else {
        println!("UNSATISFIABLE");
    }
}
