use proconio::input;

fn main() {
    input! {
        mut x: usize,
        mut y: usize,
    }

    let mut v = vec![];
    while x != y {
        v.push((x, y));
        if x > y {
            x -= y;
        } else {
            y -= x;
        }
    }

    println!("{}", v.len());
    for &(x, y) in v.iter().rev() {
        println!("{} {}", x, y);
    }
}
