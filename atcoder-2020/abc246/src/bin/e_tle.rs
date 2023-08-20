use std::{collections::{HashSet, BinaryHeap}, cmp::Reverse};

use proconio::{input, marker::{Chars, Usize1}};

fn main() {
    input! {
        n: usize,
        ax: Usize1,
        ay: Usize1,
        bx: Usize1,
        by: Usize1,
        s: [Chars; n],
    }

    if (ax + ay + bx + by) % 2 == 1 {
        println!("{}", -1);
        return;
    } 

    let mut set = HashSet::new();
    let mut query = BinaryHeap::new();
    set.insert((ax, ay));
    query.push((Reverse(0), (ax, ay), true));
    query.push((Reverse(0), (ax, ay), false));
    while let Some((Reverse(step), (x, y), dir)) = query.pop() {
        if x == bx && y == by {
            println!("{}", step);
            return;
        }

        if dir {
            for i in 1..=(x.min(y)) {
                let (x0, y0) = (x - i, y - i);
                if s[y0][x0] == '#' {
                    break;
                }
                if set.insert((x0, y0)) {
                    println!("{}, ({}, {}), ({}, {})", step + 1, x, y, x0, y0);
                    query.push((Reverse(step + 1), (x0, y0), !dir));
                }
            }

            for i in 1..=((n - x - 1).min(n - y - 1)) {
                let (x0, y0) = (x + i, y + i);
                if s[y0][x0] == '#' {
                    break;
                }
                if set.insert((x0, y0)) {
                    println!("{}, ({}, {}), ({}, {})", step + 1, x, y, x0, y0);
                    query.push((Reverse(step + 1), (x0, y0), !dir));
                }
            }
        }
        
        if !dir {
            for i in 1..=(x.min(n - y - 1)) {
                let (x0, y0) = (x - i, y + i);
                if s[y0][x0] == '#' {
                    break;
                }
                if set.insert((x0, y0)) {
                    println!("{}, ({}, {}), ({}, {})", step + 1, x, y, x0, y0);
                    query.push((Reverse(step + 1), (x0, y0), !dir));
                }
            }
    
            for i in 1..=((n - x - 1).min(y)) {
                let (x0, y0) = (x + i, y - i);
                if s[y0][x0] == '#' {
                    break;
                }
                if set.insert((x0, y0)) {
                    println!("{}, ({}, {}), ({}, {})", step + 1, x, y, x0, y0);
                    query.push((Reverse(step + 1), (x0, y0), !dir));
                }
            }
        }
    }

    println!("{}", -1);
}
