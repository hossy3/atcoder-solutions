use proconio::input;

fn f(x: usize, y: usize, a: &[usize]) -> isize {
    let mut all_x_max = 0;
    let mut all_y_min = usize::MAX;

    for &k in a {
        all_x_max = all_x_max.max(k * x);
        all_y_min = all_y_min.min(k * y);
    }
    if all_x_max > all_y_min {
        return -1;
    }

    let mut result = 0;
    for &k in a {
        if (k * y) % (y - x) != all_y_min % (y - x) {
            return -1;
        }
        result += (all_y_min - k * x) / (y - x);
    }

    result as isize
}

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        a: [usize; n],
    }
    let result = f(x, y, &a);
    println!("{result}");
}
