use std::{collections::BTreeSet, iter};

use itertools::Itertools;
use proconio::{input, marker::Chars};

const DX: &[isize] = &[1, 0, -1, 0];
const DY: &[isize] = &[0, -1, 0, 1];
const OFFSET: usize = 32;

#[derive(Clone)]
struct Node {
    parent: usize,
    dist: isize,
    local_dir: usize, // 0..4
    world_dir: usize, // 0..4
    x: isize,
    y: isize,
    holding: bool,
}

impl Node {
    fn new() -> Self {
        Self {
            parent: 0,
            dist: 0,
            local_dir: 0,
            world_dir: 0,
            x: 0,
            y: 0,
            holding: false,
        }
    }
}

#[derive(Clone)]
struct Arm {
    nodes_start: usize,
    nodes_count: usize,
    start_i: usize,
    start_dirs: Vec<usize>, // 0..4
    goal_i: usize,
    goal_dirs: Vec<usize>, // 0..4
    holding: bool,
    active: bool,
}

impl Arm {
    fn new() -> Self {
        Self {
            nodes_start: 0,
            nodes_count: 0,
            start_i: 0,
            start_dirs: vec![], // 0..4
            goal_i: 0,
            goal_dirs: vec![], // 0..4
            holding: false,
            active: true,
        }
    }
}

// nodes の world_dir, x, y を更新する
fn update_nodes(nodes: &mut [Node]) {
    for i in 0..nodes.len() {
        if i == 0 {
            nodes[i].world_dir = nodes[i].local_dir;
        } else {
            let j = nodes[i].parent;
            let dir = (nodes[i].local_dir + nodes[j].local_dir) % 4;
            nodes[i].world_dir = dir;
            nodes[i].x = nodes[j].x + DX[dir] * nodes[i].dist;
            nodes[i].y = nodes[j].y + DY[dir] * nodes[i].dist;
        }
    }
}

// 腕をのばして届く場所と、そのときのノード折り曲げ状態の2次元配列を返す
fn create_map(n: usize, arm_len: usize) -> Vec<Vec<Option<Vec<usize>>>> {
    let n0 = OFFSET * 2;
    let mut map = vec![vec![vec![None; n0]; n0]; arm_len + 1];
    map[0][OFFSET][OFFSET] = Some(vec![]); // 相対的な向き

    for i in 0..arm_len {
        let k = (1 << (arm_len - i - 1)).min(n - 1) as isize; // 長さ
        for y in 0..n0 {
            for x in 0..n0 {
                if map[i][y][x].is_none() {
                    continue;
                }
                let local_dirs = map[i][y][x].clone().unwrap();
                for local_dir in 0..4 {
                    let local_dirs: Vec<_> =
                        local_dirs.iter().chain(&[local_dir]).cloned().collect();
                    let world_dir = local_dirs.iter().sum::<usize>() % 4;
                    let x0 = x.wrapping_add_signed(DX[world_dir] * k);
                    let y0 = y.wrapping_add_signed(DY[world_dir] * k);
                    if x0 < n0 && y0 < n0 {
                        map[i + 1][y0][x0] = Some(local_dirs);
                    }
                }
            }
        }
    }

    map[arm_len].clone()
}

fn create_maps(n: usize, arm_lens: &[usize]) -> Vec<Vec<Vec<Option<Vec<usize>>>>> {
    let mut maps = vec![];
    for &arm_len in arm_lens {
        maps.push(create_map(n, arm_len));
    }
    maps
}

fn create_arm_lens(n: usize, mut v: usize) -> Vec<usize> {
    let m = if n < 3 {
        1
    } else if n < 5 {
        2
    } else if n < 9 {
        3
    } else if n < 17 {
        4
    } else {
        5
    };
    let mut arm_lens = vec![];
    v -= 1;
    while v > 0 {
        let arm_len = v.min(m);
        arm_lens.push(arm_len);
        v -= arm_len;
    }
    arm_lens
}

fn create_route(n: usize, v: usize) -> Vec<(isize, isize)> {
    let n = n as isize;
    if v > 5 || n < 17 {
        let center = (n - 1) / 2;
        vec![
            (center, center),
            (center + 1, center),
            (center + 1, center + 1),
            (center, center + 1),
        ]
    } else {
        let mut vec = vec![];
        for x in 8..=(n - 8) {
            vec.push((x, 7));
        }
        for y in 8..=(n - 8) {
            vec.push((n - 8, y));
        }
        for x in (8..=(n - 8)).rev() {
            vec.push((x, n - 7));
        }
        for y in (8..=(n - 8)).rev() {
            vec.push((8, y));
        }
        vec
    }
}

fn create_scores(
    n: usize,
    maps: &[Vec<Vec<Option<Vec<usize>>>>],
    route: &[(isize, isize)],
) -> Vec<Vec<Vec<usize>>> {
    let mut scores = vec![];
    for map in maps {
        let n0 = map.len();
        let mut counts = vec![vec![0usize; n]; n];
        for &(x, y) in route {
            for y0 in 0..n0 {
                for x0 in 0..n0 {
                    let x = (x as usize + x0).wrapping_sub(OFFSET);
                    let y = (y as usize + y0).wrapping_sub(OFFSET);
                    if x < n && y < n && map[y0][x0].is_some() {
                        counts[y][x] += 1;
                    }
                }
            }
        }

        let mut score = vec![vec![0usize; n]; n];
        for y in 0..n {
            for x in 0..n {
                if counts[y][x] > 0 {
                    score[y][x] = usize::MAX / (counts[y][x] + 1) + ((x + y) * 2).abs_diff(n);
                }
            }
        }
        scores.push(score);
    }

    scores
}

fn create_set(s: &[Vec<char>]) -> BTreeSet<(isize, isize)> {
    let mut set = BTreeSet::new();
    for (y, s) in s.iter().enumerate() {
        for (x, &c) in s.iter().enumerate() {
            if c == '1' {
                set.insert((x as isize, y as isize));
            }
        }
    }
    set
}

fn optimize_set(s: &mut BTreeSet<(isize, isize)>, t: &mut BTreeSet<(isize, isize)>) {
    let mut u = BTreeSet::new();
    for &x in s.iter() {
        if t.contains(&x) {
            u.insert(x);
        }
    }
    for x in u {
        s.remove(&x);
        t.remove(&x);
    }
}

fn has_neighbor(
    (x, y): (isize, isize),
    dirs: &[usize],
    map: &[Vec<Option<Vec<usize>>>],
    set: &BTreeSet<(isize, isize)>,
    opt_xy: Option<(isize, isize)>,
) -> bool {
    let n0 = map.len();

    for &(x0, y0) in set {
        if let Some((x1, y1)) = opt_xy {
            if x1 == x0 && y1 == y0 {
                continue; // 消したあとは隣でなくなるので別扱い
            }
        }

        let x0 = OFFSET.wrapping_add_signed(x0 - x);
        let y0 = OFFSET.wrapping_add_signed(y0 - y);
        if x0 >= n0 || y0 >= n0 {
            continue;
        }
        let Some(dirs0) = &map[y0][x0] else {
            continue;
        };
        if iter::zip(dirs, dirs0).any(|(x, y)| x.abs_diff(*y) == 2) {
            continue; // 一手では動かせない
        }
        return true;
    }

    false
}

fn find_next_arm(
    i: usize,
    route: &[(isize, isize)],
    set: &BTreeSet<(isize, isize)>,
    dirs: &[usize],
    map: &[Vec<Option<Vec<usize>>>],
    scores: &[Vec<usize>],
    opt_set: Option<&BTreeSet<(isize, isize)>>,
    opt_xy: Option<(isize, isize)>,
) -> Option<(usize, (isize, isize), Vec<usize>)> {
    let n0 = map.len();

    let j = (i + 1) % route.len();
    let (x0, y0) = route[j];
    let mut cand = None;
    for y1 in 0..n0 {
        for x1 in 0..n0 {
            let Some(dirs1) = &map[y1][x1] else {
                continue;
            };
            if iter::zip(dirs, dirs1).any(|(x, y)| x.abs_diff(*y) == 2) {
                continue; // 一手では動かせない
            }
            let (x1, y1) = (
                x0 + x1 as isize - OFFSET as isize,
                y0 + y1 as isize - OFFSET as isize,
            );
            if !set.contains(&(x1, y1)) {
                continue;
            }

            let score = scores[y1 as usize][x1 as usize];
            let prior = if opt_set.is_some() {
                let (x1, y1) = route[(j + 1) % route.len()];
                has_neighbor((x1, y1), &dirs1, &map, opt_set.unwrap(), opt_xy)
            } else {
                false
            };
            if let Some((_, _, _, score0, prior0)) = cand {
                if !prior && prior0 {
                    continue;
                }
                if score < score0 {
                    continue;
                }
            }
            cand = Some((x1, y1, dirs1, score, prior));
        }
    }
    if let Some((x, y, v, _, _)) = cand {
        // eprintln!("{}, ({}, {}), {:?}", j, x, y, &v);
        return Some((j, (x, y), v.clone()));
    }

    for j in 0..route.len() {
        let j = (i + j + 2) % route.len();
        let (x0, y0) = route[j];
        let mut cand = None;
        for y1 in 0..n0 {
            for x1 in 0..n0 {
                let Some(dirs1) = &map[y1][x1] else {
                    continue;
                };
                let (x1, y1) = (
                    x0 + x1 as isize - OFFSET as isize,
                    y0 + y1 as isize - OFFSET as isize,
                );
                if !set.contains(&(x1, y1)) {
                    continue;
                }

                let score = scores[y1 as usize][x1 as usize];
                let prior = if opt_set.is_some() {
                    let (x1, y1) = route[(j + 1) % route.len()];
                    has_neighbor((x1, y1), &dirs1, &map, opt_set.unwrap(), opt_xy)
                } else {
                    false
                };
                if let Some((_, _, _, score0, prior0)) = cand {
                    if !prior && prior0 {
                        continue;
                    }
                    if score < score0 {
                        continue;
                    }
                }
                cand = Some((x1, y1, dirs1, score, prior));
            }
        }
        if let Some((x, y, v, _, _)) = cand {
            // eprintln!("{}, ({}, {}), {:?}", j, x, y, &v);
            return Some((j, (x, y), v.clone()));
        }
    }

    None
}

fn find_next_arm_ex(
    i: usize,
    route: &[(isize, isize)],
    set: &BTreeSet<(isize, isize)>,
    dirs: &[usize],
    map: &[Vec<Option<Vec<usize>>>],
    scores: &[Vec<usize>],
) -> Option<(usize, (isize, isize), Vec<usize>)> {
    let Some((mut i1, (mut x1, mut y1), mut dirs1)) = find_next_arm(i, route, set, dirs, map, scores, None, None) else {
        return None;
    };

    // 最後の1つのときは逆回りも調べる
    let i = i % route.len();
    let mut route2 = vec![];
    let mut len1 = (i1 + route.len() - i) % route.len();
    for j in 0..=len1 {
        route2.push(route[(i + route.len() - j) % route.len()]);
    }
    if let Some((i2, (x2, y2), dirs2)) =
        find_next_arm(i, &route2, set, dirs, map, scores, None, None)
    {
        if 0 < i2 && i2 < len1 {
            i1 = (i + route.len() - i2) % route.len();
            (len1, (x1, y1), dirs1) = (i2, (x2, y2), dirs2);
        }
    };
    let (len1, (x1, y1), dirs1) = (len1, (x1, y1), dirs1);

    let (x, y) = route[i % route.len()];
    if len1 > 1 {
        let v = vec![(x, y), (x, y)];
        if let Some((i2, (x2, y2), dirs2)) =
            find_next_arm(0, &v, &set, &dirs, &map, scores, None, None)
        {
            if i2 == 1 {
                return Some((i, (x2, y2), dirs2));
            }
        }
    }

    if len1 > 2 {
        let i0 = (i + 1) % route.len();
        let (x0, y0) = route[i0];
        let v = vec![(x, y), (x0, y0), (x0, y0)];
        if let Some((i2, (x2, y2), dirs2)) =
            find_next_arm(0, &v, &set, &dirs, &map, scores, None, None)
        {
            if i2 == 2 {
                return Some((i0, (x2, y2), dirs2));
            }
        }

        let i0 = (i + route.len() - 1) % route.len();
        let (x0, y0) = route[i0];
        let v = vec![(x, y), (x0, y0), (x0, y0)];
        if let Some((i2, (x2, y2), dirs2)) =
            find_next_arm(0, &v, &set, &dirs, &map, scores, None, None)
        {
            if i2 == 2 {
                return Some((i0, (x2, y2), dirs2));
            }
        }
    }

    Some((i1, (x1, y1), dirs1))
}

fn get_move_char((x, y): (isize, isize), (next_x, next_y): (isize, isize)) -> char {
    if y < next_y {
        'D'
    } else if x < next_x {
        'R'
    } else if y > next_y {
        'U'
    } else if x > next_x {
        'L'
    } else {
        '.'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_arm_lens() {
        assert_eq!(create_arm_lens(16, 5), vec![4]);
        assert_eq!(create_arm_lens(16, 6), vec![4, 1]);
        assert_eq!(create_arm_lens(16, 7), vec![4, 2]);
        assert_eq!(create_arm_lens(16, 12), vec![4, 4, 3]);
        assert_eq!(create_arm_lens(16, 15), vec![4, 4, 4, 2]);

        assert_eq!(create_arm_lens(17, 5), vec![4]);
        assert_eq!(create_arm_lens(17, 6), vec![5]);
        assert_eq!(create_arm_lens(17, 7), vec![5, 1]);
        assert_eq!(create_arm_lens(17, 12), vec![5, 5, 1]);
        assert_eq!(create_arm_lens(17, 15), vec![5, 5, 4]);
    }

    #[test]
    fn test_create_map_1() {
        let map = create_map(15, 1);
        assert_eq!(map[OFFSET][OFFSET], None);
        assert_eq!(map[OFFSET + 1][OFFSET], Some(vec![0]));
        assert_eq!(map[OFFSET][OFFSET + 1], Some(vec![1]));
        assert_eq!(map[OFFSET - 1][OFFSET], Some(vec![2]));
        assert_eq!(map[OFFSET][OFFSET - 1], Some(vec![3]));
    }

    #[test]
    fn test_create_map_2() {
        let map = create_map(15, 2);
        assert_eq!(map[OFFSET][OFFSET], None);
        assert_eq!(map[OFFSET + 3][OFFSET], Some(vec![0, 0]));
        assert_eq!(map[OFFSET + 2][OFFSET + 1], Some(vec![0, 1]));
        assert_eq!(map[OFFSET + 1][OFFSET], Some(vec![0, 2]));
        assert_eq!(map[OFFSET + 2][OFFSET - 1], Some(vec![0, 3]));

        assert_eq!(map[OFFSET][OFFSET + 3], Some(vec![1, 0]));
        assert_eq!(map[OFFSET - 1][OFFSET + 2], Some(vec![1, 1]));
        assert_eq!(map[OFFSET][OFFSET + 1], Some(vec![1, 2]));
        assert_eq!(map[OFFSET + 1][OFFSET + 2], Some(vec![1, 3]));

        assert_eq!(map[OFFSET - 3][OFFSET], Some(vec![2, 0]));
        assert_eq!(map[OFFSET][OFFSET - 3], Some(vec![3, 0]));
    }

    #[test]
    fn test_create_route() {
        assert_eq!(create_route(15, 5), vec![(7, 7), (8, 7), (8, 8), (7, 8)]);
        assert_eq!(create_route(16, 5), vec![(7, 7), (8, 7), (8, 8), (7, 8)]);
        assert_eq!(
            create_route(17, 5),
            vec![
                (8, 7),
                (9, 7),
                (9, 8),
                (9, 9),
                (9, 10),
                (8, 10),
                (8, 9),
                (8, 8)
            ]
        );
        assert_eq!(
            create_route(30, 6),
            vec![(14, 14), (15, 14), (15, 15), (14, 15)]
        );
    }
}

fn output_arms(nodes: &[Node]) {
    println!("{}", nodes.len());
    for node in &nodes[1..] {
        println!("{} {}", node.parent, node.dist);
    }

    // 出力の (x, y) は (row, col)。実装は (col, row)
    println!("{} {}", nodes[0].y, nodes[0].x);
}

fn process(
    maps: &[Vec<Vec<Option<Vec<usize>>>>],
    route: &[(isize, isize)],
    scores: &[Vec<Vec<usize>>],
    i: &mut usize,
    nodes: &mut [Node],
    arms: &mut [Arm],
    set_s: &mut BTreeSet<(isize, isize)>,
    set_t: &mut BTreeSet<(isize, isize)>,
) -> Option<Vec<char>> {
    let v = nodes.len();
    let mut result = vec!['.'; v * 2];

    let i0 = *i + 1;
    let (x, y) = route[*i % route.len()];
    let (x0, y0) = route[i0 % route.len()]; // 次の位置

    result[0] = get_move_char((x, y), (x0, y0));
    nodes[0].x = x0;
    nodes[0].y = y0;

    for j in 0..arms.len() {
        let map = &maps[j];
        let scores = &scores[j];
        let arm = &mut arms[j];
        if !arm.active {
            continue;
        }
        let arm_len = arm.nodes_count;
        let arm_start_node = arm.nodes_start;

        let mut dirs = vec![];
        for k in 0..arm_len {
            dirs.push(nodes[arm_start_node + k].local_dir);
        }
        if arm.start_dirs.is_empty() && arm.goal_dirs.is_empty() {
            if let Some((i1, (x1, y1), dirs1)) =
                find_next_arm(*i, route, set_s, &dirs, map, scores, Some(set_t), None)
            {
                let opt_xy = Some((x1, y1));
                if let Some((i2, (x2, y2), dirs2)) =
                    find_next_arm(i1, route, set_t, &dirs1, map, scores, Some(set_s), opt_xy)
                {
                    arm.start_i = i1;
                    arm.start_dirs = dirs1;
                    arm.goal_i = i2;
                    arm.goal_dirs = dirs2;
                    set_s.remove(&(x1, y1));
                    set_t.remove(&(x2, y2));
                } else {
                    arm.active = false;
                    continue;
                }
            } else {
                arm.active = false;
                continue;
            }
        }

        let (x1, y1) = if !arm.holding {
            route[arm.start_i]
        } else {
            route[arm.goal_i]
        };
        postprocess_arm((x0, y0), (x1, y1), nodes, arm, &mut result);
    }

    *i = i0;
    Some(result)
}

// 残り1つの場合は動きを減らす
fn process_1node(
    map: &[Vec<Option<Vec<usize>>>],
    route: &[(isize, isize)],
    scores: &[Vec<usize>],
    i: &mut usize,
    nodes: &mut [Node],
    arm: &mut Arm,
    set_s: &mut BTreeSet<(isize, isize)>,
    set_t: &mut BTreeSet<(isize, isize)>,
) -> Option<Vec<char>> {
    let v = nodes.len();
    let mut result = vec!['.'; v * 2];

    let arm_len = arm.nodes_count;
    let arm_start_node = arm.nodes_start;
    let mut dirs = vec![];
    for k in 0..arm_len {
        dirs.push(nodes[arm_start_node + k].local_dir);
    }

    if !arm.holding && arm.start_dirs.is_empty() {
        if let Some((i1, (x1, y1), dirs1)) = find_next_arm_ex(*i, route, set_s, &dirs, map, scores)
        {
            arm.start_i = i1;
            arm.start_dirs = dirs1;
            set_s.remove(&(x1, y1));
        } else {
            arm.active = false;
            return None;
        }
    } else if arm.holding && arm.goal_dirs.is_empty() {
        if let Some((i1, (x1, y1), dirs1)) = find_next_arm_ex(*i, route, set_t, &dirs, map, scores)
        {
            arm.goal_i = i1;
            arm.goal_dirs = dirs1;
            set_t.remove(&(x1, y1));
        } else {
            arm.active = false;
            return None;
        }
    }

    let i1 = if !arm.holding {
        arm.start_i
    } else {
        arm.goal_i
    };
    let (x, y) = (nodes[0].x, nodes[0].y); // 今の場所
    let (x1, y1) = route[i1];
    let (x0, y0) = if x1 < x {
        (x - 1, y)
    } else if x1 > x {
        (x + 1, y)
    } else if y1 < y {
        (x, y - 1)
    } else if y1 > y {
        (x, y + 1)
    } else {
        (x, y)
    }; // 次の位置

    result[0] = get_move_char((x, y), (x0, y0));
    nodes[0].x = x0;
    nodes[0].y = y0;

    postprocess_arm((x0, y0), (x1, y1), nodes, arm, &mut result);
    *i = i1;
    Some(result)
}

fn postprocess_arm(
    (x0, y0): (isize, isize),
    (x1, y1): (isize, isize),
    nodes: &mut [Node],
    arm: &mut Arm,
    result: &mut [char],
) {
    let v = nodes.len();
    let arm_len = arm.nodes_count;
    let arm_start_node = arm.nodes_start;

    if !arm.holding {
        for k in 0..arm_len {
            let k0 = arm_start_node + k;
            let dir0 = nodes[k0].local_dir;
            let dir1 = arm.start_dirs[k];
            if (dir0 + 3) % 4 == dir1 {
                nodes[k0].local_dir = (dir0 + 3) % 4;
                result[k0] = 'R';
            } else if dir0 != dir1 {
                nodes[k0].local_dir = (dir0 + 1) % 4;
                result[k0] = 'L';
            }
        }
        let same_dirs =
            (0..arm_len).all(|k| nodes[arm_start_node + k].local_dir == arm.start_dirs[k]);
        if x0 == x1 && y0 == y1 && same_dirs {
            let k0 = arm_start_node + arm_len - 1;
            nodes[k0].holding = true;
            result[v + k0] = 'P';
            arm.start_dirs.clear();
            arm.holding = true;
        }
    } else {
        for k in 0..arm_len {
            let k0 = arm_start_node + k;
            let dir0 = nodes[k0].local_dir;
            let dir1 = arm.goal_dirs[k];
            if (dir0 + 3) % 4 == dir1 {
                nodes[k0].local_dir = (dir0 + 3) % 4;
                result[k0] = 'R';
            } else if dir0 != dir1 {
                nodes[k0].local_dir = (dir0 + 1) % 4;
                result[k0] = 'L';
            }
        }
        let same_dirs =
            (0..arm_len).all(|k| nodes[arm_start_node + k].local_dir == arm.goal_dirs[k]);
        if x0 == x1 && y0 == y1 && same_dirs {
            let k0 = arm_start_node + arm_len - 1;
            nodes[k0].holding = false;
            result[v + k0] = 'P';
            arm.goal_dirs.clear();
            arm.holding = false;
        }
    }
}

fn process_all(
    maps: &[Vec<Vec<Option<Vec<usize>>>>],
    route: &[(isize, isize)],
    scores: &[Vec<Vec<usize>>],
    nodes: &mut [Node],
    arms: &mut [Arm],
    set_s_org: &BTreeSet<(isize, isize)>,
    set_t_org: &BTreeSet<(isize, isize)>,
) -> ((isize, isize), Vec<Vec<char>>) {
    let mut cand_xy: Option<(isize, isize)> = None;
    let mut cand_results = vec![];
    let arms_len = arms.len();

    let mut f = |route: &[(isize, isize)]| {
        for i in 0..route.len() {
            // 順方向
            (nodes[0].x, nodes[0].y) = route[i];
            for node in nodes.iter_mut() {
                node.local_dir = 0;
            }
            update_nodes(nodes);
            for arm in arms.iter_mut() {
                arm.active = true;
                arm.holding = false;
            }

            let mut results = vec![];
            let mut i0 = i; // 今の場所

            let mut set_s = set_s_org.clone();
            let mut set_t = set_t_org.clone();
            for _ in 0..10000 {
                if set_s.is_empty() && set_t.is_empty() && arms.iter().all(|arm| !arm.active) {
                    break;
                }

                if arms.iter().filter(|arm| arm.active).count() == 1 {
                    let Some((j, _)) = arms.iter().find_position(|arm| arm.active) else { unreachable!() };
                    if let Some(result) = process_1node(
                        &maps[j],
                        &route,
                        &scores[j],
                        &mut i0,
                        nodes,
                        &mut arms[j],
                        &mut set_s,
                        &mut set_t,
                    ) {
                        results.push(result);
                    }
                } else {
                    if let Some(result) = process(
                        &maps, &route, &scores, &mut i0, nodes, arms, &mut set_s, &mut set_t,
                    ) {
                        results.push(result);
                    }
                }
            }

            while results.len() > 0 && !results[results.len() - 1].iter().contains(&'P') {
                results.pop();
            }

            if cand_xy.is_some() && cand_results.len() < results.len() {
                continue;
            }
            cand_xy = Some(route[i]);
            cand_results = results;
        }
    };

    f(route);
    if arms_len > 1 {
        let route: Vec<_> = route.iter().rev().cloned().collect();
        f(&route);
    }

    let Some(xy) = cand_xy else { unreachable!() };
    (xy, cand_results)
}

fn main() {
    input! {
        (n, _m, v): (usize, usize, usize),
        s: [Chars; n],
        t: [Chars; n],
    }

    let route = create_route(n, v);
    let arm_lens = create_arm_lens(n, v);
    let arms_num = arm_lens.len();
    let maps = create_maps(n, &arm_lens);
    let scores = create_scores(n, &maps, &route);
    let mut arm_start_nodes = vec![1usize];
    for &len in &arm_lens {
        let Some(&cur) = arm_start_nodes.last() else { unreachable!() };
        arm_start_nodes.push(cur + len);
    }

    let mut nodes = vec![Node::new(); v];
    (nodes[0].x, nodes[0].y) = route[0];
    for i in 0..arms_num {
        let arm_len = arm_lens[i];
        let arm_start_node = arm_start_nodes[i];
        for j in 0..arm_len {
            let k = arm_start_node + j;
            nodes[k].parent = if j == 0 { 0 } else { k - 1 };
            nodes[k].dist = (1 << (arm_len - j - 1)).min(n - 1) as isize;
        }
    }
    update_nodes(&mut nodes);

    let mut set_s = create_set(&s);
    let mut set_t = create_set(&t);
    optimize_set(&mut set_s, &mut set_t);

    let mut arms = vec![Arm::new(); arms_num];
    for (i, arm) in arms.iter_mut().enumerate() {
        arm.nodes_start = arm_start_nodes[i];
        arm.nodes_count = arm_lens[i];
    }

    let (xy, results) = process_all(
        &maps, &route, &scores, &mut nodes, &mut arms, &set_s, &set_t,
    );
    (nodes[0].x, nodes[0].y) = xy;

    // 別のアームでも調べる
    if arm_lens.len() > 1 && arm_lens[1] == 5 {
        let arm_lens = match v {
            11 => vec![5, 4, 1],
            12 => vec![5, 4, 2],
            13 => vec![5, 4, 3],
            14 => vec![5, 4, 4],
            15 => vec![5, 4, 4, 1],
            _ => arm_lens.clone(),
        };
        let arms_num = arm_lens.len();
        let maps = create_maps(n, &arm_lens);
        let scores = create_scores(n, &maps, &route);
        let mut arm_start_nodes = vec![1usize];
        for &len in &arm_lens {
            let Some(&cur) = arm_start_nodes.last() else { unreachable!() };
            arm_start_nodes.push(cur + len);
        }

        let mut nodes = vec![Node::new(); v];
        (nodes[0].x, nodes[0].y) = route[0];
        for i in 0..arms_num {
            let arm_len = arm_lens[i];
            let arm_start_node = arm_start_nodes[i];
            for j in 0..arm_len {
                let k = arm_start_node + j;
                nodes[k].parent = if j == 0 { 0 } else { k - 1 };
                nodes[k].dist = (1 << (arm_len - j - 1)).min(n - 1) as isize;
            }
        }
        update_nodes(&mut nodes);

        let mut set_s = create_set(&s);
        let mut set_t = create_set(&t);
        optimize_set(&mut set_s, &mut set_t);

        let mut arms = vec![Arm::new(); arms_num];
        for (i, arm) in arms.iter_mut().enumerate() {
            arm.nodes_start = arm_start_nodes[i];
            arm.nodes_count = arm_lens[i];
        }

        let (xy0, results0) = process_all(
            &maps, &route, &scores, &mut nodes, &mut arms, &set_s, &set_t,
        );
        (nodes[0].x, nodes[0].y) = xy0;

        if results0.len() < results.len() {
            output_arms(&nodes);
            for result in results0 {
                println!("{}", result.iter().join(""));
            }
            return;
        }
    }

    // 別のルートでも調べる
    if route.len() > 4 {
        let route: Vec<_> = route.iter().map(|&(x, y)| (y, x)).collect();
        let scores = create_scores(n, &maps, &route);

        let mut nodes = vec![Node::new(); v];
        (nodes[0].x, nodes[0].y) = route[0];
        for i in 0..arms_num {
            let arm_len = arm_lens[i];
            let arm_start_node = arm_start_nodes[i];
            for j in 0..arm_len {
                let k = arm_start_node + j;
                nodes[k].parent = if j == 0 { 0 } else { k - 1 };
                nodes[k].dist = (1 << (arm_len - j - 1)).min(n - 1) as isize;
            }
        }
        update_nodes(&mut nodes);

        let mut set_s = create_set(&s);
        let mut set_t = create_set(&t);
        optimize_set(&mut set_s, &mut set_t);

        let mut arms = vec![Arm::new(); arms_num];
        for (i, arm) in arms.iter_mut().enumerate() {
            arm.nodes_start = arm_start_nodes[i];
            arm.nodes_count = arm_lens[i];
        }

        let (xy0, results0) = process_all(
            &maps, &route, &scores, &mut nodes, &mut arms, &set_s, &set_t,
        );
        (nodes[0].x, nodes[0].y) = xy0;

        if results0.len() < results.len() {
            output_arms(&nodes);
            for result in results0 {
                println!("{}", result.iter().join(""));
            }
            return;
        }
    } else if arm_lens[0] == 5 {
        for dy in 0isize..1 {
            for dx in 0isize..1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let route: Vec<_> = route.iter().map(|&(x, y)| (x + dx, y + dy)).collect();
                let scores = create_scores(n, &maps, &route);

                let mut nodes = vec![Node::new(); v];
                (nodes[0].x, nodes[0].y) = route[0];
                for i in 0..arms_num {
                    let arm_len = arm_lens[i];
                    let arm_start_node = arm_start_nodes[i];
                    for j in 0..arm_len {
                        let k = arm_start_node + j;
                        nodes[k].parent = if j == 0 { 0 } else { k - 1 };
                        nodes[k].dist = (1 << (arm_len - j - 1)).min(n - 1) as isize;
                    }
                }
                update_nodes(&mut nodes);

                let mut set_s = create_set(&s);
                let mut set_t = create_set(&t);
                optimize_set(&mut set_s, &mut set_t);

                let mut arms = vec![Arm::new(); arms_num];
                for (i, arm) in arms.iter_mut().enumerate() {
                    arm.nodes_start = arm_start_nodes[i];
                    arm.nodes_count = arm_lens[i];
                }

                let (xy0, results0) = process_all(
                    &maps, &route, &scores, &mut nodes, &mut arms, &set_s, &set_t,
                );
                (nodes[0].x, nodes[0].y) = xy0;

                if results0.len() < results.len() {
                    output_arms(&nodes);
                    for result in results0 {
                        println!("{}", result.iter().join(""));
                    }
                    return;
                }
            }
        }
    }

    output_arms(&nodes);
    for result in results {
        println!("{}", result.iter().join(""));
    }
}
