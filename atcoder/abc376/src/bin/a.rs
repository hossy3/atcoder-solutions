use proconio::input;

fn main() {
    input! {
        n: usize,
        c: usize,
        t: [usize; n],
    }
    let mut next_time = 0;
    let mut result = 0;
    for cur_time in t {
        if next_time <= cur_time {
            next_time = cur_time + c;
            result += 1;
        }
    }
    println!("{result}");
}
