use std::collections::{HashMap, VecDeque};

use proconio::{input, marker::Chars};

fn get_players(s: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut v = vec![];
    for (i, s) in s.iter().enumerate() {
        for (j, &c) in s.iter().enumerate() {
            if c == 'P' {
                v.push((i, j));
            }
        }
    }
    v
}

fn next_pos((i, j): (usize, usize), (di, dj): (isize, isize), s: &[Vec<char>]) -> (usize, usize) {
    let i0 = i.wrapping_add_signed(di);
    let j0 = j.wrapping_add_signed(dj);
    if i0 >= s.len() || j0 >= s.len() || s[i0][j0] == '#' {
        (i, j)
    } else {
        (i0, j0)
    }
}

fn f(players: &Vec<(usize, usize)>, s: &[Vec<char>]) -> i64 {
    let mut map = HashMap::new();
    map.insert(
        ((players[0].0, players[0].1), (players[1].0, players[1].1)),
        0,
    );

    let mut queue = VecDeque::new();
    queue.push_back((
        (players[0].0, players[0].1),
        (players[1].0, players[1].1),
        1usize,
    ));

    while let Some(((i0, j0), (i1, j1), step)) = queue.pop_front() {
        for (di, dj) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let (i0, j0) = next_pos((i0, j0), (di, dj), s);
            let (i1, j1) = next_pos((i1, j1), (di, dj), s);
            if (i0, j0) == (i1, j1) {
                return step as i64;
            }
            let key = ((i0, j0), (i1, j1));
            if !map.contains_key(&key) || *map.get(&key).unwrap() > step {
                (*map.entry(key).or_insert(usize::MAX)) = step;
                queue.push_back(((i0, j0), (i1, j1), step + 1));
            }
        }
    }

    -1
}

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let players = get_players(&s);
    let result = f(&players, &s);
    println!("{result}");
}
