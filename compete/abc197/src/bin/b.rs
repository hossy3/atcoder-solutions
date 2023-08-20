use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize,
        w: usize,
        x: Usize1,
        y: Usize1,
        s: [Chars; h],
    }

    let mut count = 1usize;
    for x in (0..x).rev() {
        if s[x][y] == '#' {
            break;
        }
        count += 1;
    }
    for x in (x + 1)..h {
        if s[x][y] == '#' {
            break;
        }
        count += 1;
    }
    for y in (0..y).rev() {
        if s[x][y] == '#' {
            break;
        }
        count += 1;
    }
    for y in (y + 1)..w {
        if s[x][y] == '#' {
            break;
        }
        count += 1;
    }

    println!("{}", count);
}
