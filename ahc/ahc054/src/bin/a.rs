use std::collections::{BTreeSet, HashSet, VecDeque};

use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input_interactive, marker::Chars};

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // 上下左右

// ローカルテスト用 次に進む場所の情報を返す
fn get_next_state(
    confirmed: &[Vec<bool>],
    walls: &[Vec<bool>],
    pi: usize,
    pj: usize,
    ti: usize,
    tj: usize,
    q: &[(usize, usize)],
    mut qk: usize,
) -> (usize, usize, Vec<(usize, usize)>, usize) {
    let n = confirmed.len();

    // 2, 3
    let mut xy = vec![];
    if !confirmed[pi][pj] {
        xy.push((pi, pj));
    }
    let mut coming_walls = HashSet::new();
    for &(di, dj) in &DIRS {
        let mut i = pi;
        let mut j = pj;
        loop {
            i = i.wrapping_add_signed(di);
            j = j.wrapping_add_signed(dj);
            if i >= n || j >= n {
                break;
            }
            if confirmed[i][j] {
                if walls[i][j] {
                    break;
                }
                continue;
            }
            xy.push((i, j));
            if i == ti && j == tj {
                qk = q.iter().position(|&q| q == (ti, tj)).unwrap(); // 目的地を変える
            }
            if walls[i][j] {
                coming_walls.insert((i, j));
                break;
            }
        }
    }

    // 4 目的地が確認済みマスでない場合、暫定地図で到達可能か確認する
    // 5 目的地がゴール以外の確認済みマスの場合、目的地を切り替える
    while q[qk] != (ti, tj) {
        let (i, j) = q[qk];
        if confirmed[i][j] || xy.contains(&(i, j)) {
            qk += 1;
            continue;
        }

        let mut dsu = Dsu::new(n * n);
        let f = |i: usize, j: usize| {
            i < n && j < n && !((confirmed[i][j] && walls[i][j]) || coming_walls.contains(&(i, j)))
        };
        let g = |i: usize, j: usize| i * n + j;

        for i in 0..n {
            for j in 0..n {
                if !f(i, j) {
                    continue;
                }
                for (di, dj) in [(1, 0), (0, 1)] {
                    let i0 = i + di;
                    let j0 = j + dj;
                    if !f(i0, j0) {
                        continue;
                    }
                    dsu.merge(g(i, j), g(i0, j0));
                }
            }
        }

        if dsu.same(g(pi, pj), g(i, j)) {
            // if pi == 14 && pj == 11 {
            //     eprintln!();
            // }
            break; // 到達可能
        }
        qk += 1;
    }

    // 6 最短距離を計算する
    let mut map = vec![vec![usize::MAX; n]; n];
    let (i, j) = q[qk];
    map[i][j] = 0;
    let mut queue = VecDeque::new();
    queue.push_back((i, j));

    while let Some((pi, pj)) = queue.pop_front() {
        for &(di, dj) in &DIRS {
            let i = pi.wrapping_add_signed(di);
            let j = pj.wrapping_add_signed(dj);
            if i >= n || j >= n {
                continue;
            }
            if map[i][j] <= map[pi][pj] + 1 {
                continue;
            }
            if (confirmed[i][j] && walls[i][j]) || coming_walls.contains(&(i, j)) {
                continue;
            }
            map[i][j] = map[pi][pj] + 1;
            queue.push_back((i, j));
        }
    }

    // eprintln!("{} {}", pi, pj);
    // eprintln!("{:?}", &coming_walls);

    let mut ij: Option<(usize, usize)> = None;
    for &(di, dj) in &DIRS {
        let i = pi.wrapping_add_signed(di);
        let j = pj.wrapping_add_signed(dj);
        if i >= n || j >= n {
            continue;
        }
        if map[i][j] == usize::MAX {
            continue;
        }
        if let Some((i0, j0)) = ij {
            if map[i][j] >= map[i0][j0] {
                continue;
            }
        }
        // eprintln!("{} {} {} {} {} {}", i, j, map[i][j], pi, pj, map[pi][pj]);
        ij = Some((i, j));
    }
    // eprintln!("{} {}", i, j);
    let Some((pi, pj)) = ij else { unreachable!() };

    (pi, pj, xy, qk)
}

// 近づく方を優先として返す
fn get_dirs_with_priority(pi: usize, pj: usize, ti: usize, tj: usize) -> [usize; 4] {
    if pi < ti {
        if pj <= tj {
            [1, 3, 2, 0]
        } else {
            [1, 2, 3, 0]
        }
    } else if pi > ti {
        if pj <= tj {
            [0, 3, 2, 1]
        } else {
            [0, 2, 3, 1]
        }
    } else {
        if pj <= tj {
            [3, 1, 0, 2]
        } else {
            [3, 0, 1, 2]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_dirs_with_priority() {
        assert_eq!(get_dirs_with_priority(0, 1, 4, 5), [1, 3, 2, 0]); // 下右 左上
    }
}

// 袋小路ができないか
fn can_add_wall(walls: &[Vec<bool>], pi: usize, pj: usize, wi: usize, wj: usize) -> bool {
    let n = walls.len();
    let mut dsu = Dsu::new(n * n);
    let f = |i: usize, j: usize| i < n && j < n && !walls[i][j] && !(i == wi && j == wj);
    let g = |i: usize, j: usize| i * n + j;

    for i in 0..n {
        for j in 0..n {
            if !f(i, j) {
                continue;
            }
            for (di, dj) in [(1, 0), (0, 1)] {
                let i0 = i + di;
                let j0 = j + dj;
                if !f(i0, j0) {
                    continue;
                }
                dsu.merge(g(i, j), g(i0, j0));
            }
        }
    }

    (0..n).all(|i| {
        (0..n).all(|j| walls[i][j] || (i == wi && j == wj) || dsu.same(g(i, j), g(pi, pj)))
    })
}

fn can_add_wall_2(
    walls: &[Vec<bool>],
    pi: usize,
    pj: usize,
    wi: usize,
    wj: usize,
    wall_set: &HashSet<(usize, usize)>,
) -> bool {
    let n = walls.len();
    let mut dsu = Dsu::new(n * n);
    let is_wall = |i: usize, j: usize| walls[i][j] || wall_set.contains(&(i, j));
    let f = |i: usize, j: usize| i < n && j < n && !is_wall(i, j) && !(i == wi && j == wj);
    let g = |i: usize, j: usize| i * n + j;

    for i in 0..n {
        for j in 0..n {
            if !f(i, j) {
                continue;
            }
            for (di, dj) in [(1, 0), (0, 1)] {
                let i0 = i + di;
                let j0 = j + dj;
                if !f(i0, j0) {
                    continue;
                }
                dsu.merge(g(i, j), g(i0, j0));
            }
        }
    }

    (0..n).all(|i| {
        (0..n).all(|j| is_wall(i, j) || (i == wi && j == wj) || dsu.same(g(i, j), g(pi, pj)))
    })
}

// (xi, xj) に到達可能か。ここは wall_set でも良い
fn can_reach_goal(
    walls: &[Vec<bool>],
    pi: usize,
    pj: usize,
    xi: usize,
    xj: usize,
    wall_set: &HashSet<(usize, usize)>,
) -> bool {
    let n = walls.len();
    let mut dsu = Dsu::new(n * n);
    let f = |i: usize, j: usize| {
        i < n && j < n && !walls[i][j] && !(wall_set.contains(&(i, j)) && (i, j) != (xi, xj))
    };
    let g = |i: usize, j: usize| i * n + j;

    for i in 0..n {
        for j in 0..n {
            if !f(i, j) {
                continue;
            }
            for (di, dj) in [(1, 0), (0, 1)] {
                let i0 = i + di;
                let j0 = j + dj;
                if !f(i0, j0) {
                    continue;
                }
                dsu.merge(g(i, j), g(i0, j0));
            }
        }
    }

    dsu.same(g(pi, pj), g(xi, xj))
}

// 袋小路を壁として追加する
fn fill_wall(walls: &[Vec<bool>], pi: usize, pj: usize, wall_set: &mut HashSet<(usize, usize)>) {
    let n = walls.len();
    let mut dsu = Dsu::new(n * n);
    let f = |i: usize, j: usize| i < n && j < n && !walls[i][j] && !wall_set.contains(&(i, j));
    let g = |i: usize, j: usize| i * n + j;

    for i in 0..n {
        for j in 0..n {
            if !f(i, j) {
                continue;
            }
            for (di, dj) in [(1, 0), (0, 1)] {
                let i0 = i + di;
                let j0 = j + dj;
                if !f(i0, j0) {
                    continue;
                }
                dsu.merge(g(i, j), g(i0, j0));
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if !walls[i][j] && !dsu.same(g(i, j), g(pi, pj)) {
                wall_set.insert((i, j));
            }
        }
    }
}

fn get_road(
    walls: &[Vec<bool>],
    wall_set: &HashSet<(usize, usize)>,
    (ti, tj): (usize, usize),
) -> Vec<Vec<bool>> {
    let n = walls.len();
    let mut road = vec![vec![false; n]; n]; // 通れる道

    let mut stack = vec![(ti, tj)];
    let mut checked = vec![vec![false; n]; n];
    checked[ti][tj] = true;

    let is_wall = |i: usize, j: usize| walls[i][j] || wall_set.contains(&(i, j));

    while let Some((i, j)) = stack.pop() {
        for di0 in [-1, 0, 1] {
            let i = i.wrapping_add_signed(di0);
            if i >= n {
                continue;
            }
            for dj0 in [-1, 0, 1] {
                if di0 == 0 && dj0 == 0 {
                    continue;
                }
                let j = j.wrapping_add_signed(dj0);
                if j >= n {
                    continue;
                }
                if checked[i][j] {
                    continue;
                }
                if !is_wall(i, j) {
                    continue;
                }
                stack.push((i, j));
                checked[i][j] = true;

                // 壁の周囲の道は通行可能とする
                for di1 in [-1, 0, 1] {
                    let i = i.wrapping_add_signed(di1);
                    if i >= n {
                        continue;
                    }
                    for dj1 in [-1, 0, 1] {
                        if di1 == 0 && dj1 == 0 {
                            continue;
                        }
                        let j = j.wrapping_add_signed(dj1);
                        if j >= n {
                            continue;
                        }
                        if checked[i][j] {
                            continue;
                        }
                        if is_wall(i, j) {
                            continue;
                        }
                        road[i][j] = true;
                        checked[i][j] = true;
                    }
                }
            }
        }
    }

    road
}

// 袋小路を作るを減らす
fn reduce_wall_set(
    walls: &[Vec<bool>],
    wall_set: &mut HashSet<(usize, usize)>,
    (pi, pj): (usize, usize),
    (ti, tj): (usize, usize),
) {
    let n = walls.len();
    let mut new_wall_set = HashSet::new();

    // ゴールから見える通路のとなりのトロントは確定
    for &(di, dj) in &DIRS {
        let mut i = ti.wrapping_add_signed(-di);
        let mut j = tj.wrapping_add_signed(-dj);
        loop {
            i = i.wrapping_add_signed(di);
            j = j.wrapping_add_signed(dj);
            if i >= n || j >= n {
                break;
            }
            if walls[i][j] || wall_set.contains(&(i, j)) {
                break;
            }
            for &(di, dj) in &DIRS {
                let i = i.wrapping_add_signed(di);
                let j = j.wrapping_add_signed(dj);
                if wall_set.contains(&(i, j)) {
                    new_wall_set.insert((i, j));
                }
            }
        }
    }

    for &(wi, wj) in wall_set.iter() {
        if !new_wall_set.contains(&(wi, wj)) && can_add_wall_2(walls, pi, pj, wi, wj, &new_wall_set)
        {
            new_wall_set.insert((wi, wj));
        }
    }
    *wall_set = new_wall_set;
}

fn modify_wall_set(
    walls: &[Vec<bool>],
    wall_set: &mut HashSet<(usize, usize)>,
    (pi, pj): (usize, usize),
    (ti, tj): (usize, usize),
    (di, dj): (isize, isize),
) {
    let n = walls.len();
    let road = get_road(walls, &wall_set, (ti, tj));

    let is_wall = |i: usize, j: usize| walls[i][j] || wall_set.contains(&(i, j));
    let priority = |i: usize, j: usize| {
        if di > 0 {
            i
        } else if di < 0 {
            n - i
        } else if dj > 0 {
            j
        } else if dj < 0 {
            n - j
        } else {
            unreachable!()
        }
    }; // 小さいほど優先

    let mut set = BTreeSet::new();
    for i in 0..n {
        for j in 0..n {
            if !road[i][j] {
                continue;
            }
            for &(di0, dj0) in &DIRS {
                let i = i.wrapping_add_signed(di0);
                let j = j.wrapping_add_signed(dj0);
                if !(i < n && j < n) {
                    continue;
                };
                if is_wall(i, j) || road[i][j] || (i, j) == (ti, tj) {
                    continue;
                };
                set.insert((priority(i, j), i, j));
            }
        }
    }

    for &(_, wi, wj) in &set {
        if can_add_wall_2(walls, pi, pj, wi, wj, &wall_set) {
            wall_set.insert((wi, wj));
        } else {
            // 同じ優先度で隣のものがあればかわりに使用する
            if di != 0 {
                for dj in [-1, 1] {
                    let mut wj = wj.wrapping_add_signed(dj);
                    while wj < n
                        && !walls[wi][wj]
                        && !wall_set.contains(&(wi, wj))
                        && !set.contains(&(priority(wi, wj), wi, wj))
                    {
                        if can_add_wall_2(walls, pi, pj, wi, wj, &wall_set) {
                            wall_set.insert((wi, wj));
                            break;
                        }
                        wj = wj.wrapping_add_signed(dj);
                    }
                }
            } else if dj != 0 {
                for di in [-1, 1] {
                    let mut wi = wi.wrapping_add_signed(di);
                    while wi < n
                        && !walls[wi][wj]
                        && !wall_set.contains(&(wi, wj))
                        && !set.contains(&(priority(wi, wj), wi, wj))
                    {
                        if can_add_wall_2(walls, pi, pj, wi, wj, &wall_set) {
                            wall_set.insert((wi, wj));
                            break;
                        }
                        wi = wi.wrapping_add_signed(dj);
                    }
                }
            }
        }
    }
}

// うずまきの壁を作る (理想状態)
fn get_initial_spiral_walls(
    walls: &[Vec<bool>],
    pi: usize,
    pj: usize,
    ti: usize,
    tj: usize,
) -> Option<(Vec<(usize, usize)>, (usize, usize))> {
    let n = walls.len();

    // 理想的なうずまきルート
    let dirs = [
        vec![0, 3, 3, 1, 1, 2, 1, 2, 2, 0, 2, 0, 0, 0, 3],
        vec![0, 3, 3, 1, 1, 1, 2, 2, 2, 0, 2, 0, 0, 0, 3],
        vec![0, 3, 3, 1, 1, 2, 1, 2, 2, 2, 0, 0, 0, 0, 3],
        vec![0, 3, 3, 1, 1, 1, 2, 2, 2, 2, 0, 0, 0, 0, 3],
        vec![0, 3, 3, 1, 1, 2, 1, 2, 2, 0, 2, 0, 0, 0, 0, 3, 3],
        vec![0, 3, 3, 1, 1, 1, 2, 2, 2, 0, 2, 0, 0, 0, 0, 3, 3],
        vec![0, 3, 3, 1, 1, 2, 1, 2, 2, 2, 0, 0, 0, 0, 0, 3, 3],
        vec![0, 3, 3, 1, 1, 1, 2, 2, 2, 2, 0, 0, 0, 0, 0, 3, 3],
    ];

    for dirs in &dirs {
        for k in 0..8 {
            let mut path_set = HashSet::new();
            path_set.insert((ti, tj));

            let mut i = ti;
            let mut j = tj;
            for &dir in dirs {
                let (di, dj) = DIRS[dir];
                let di = if k % 2 == 0 { di } else { -di };
                let dj = if (k / 2) % 2 == 0 { dj } else { -dj };
                let (di, dj) = if k / 4 == 0 { (di, dj) } else { (dj, di) };
                i = i.wrapping_add_signed(di);
                j = j.wrapping_add_signed(dj);
                if i >= n || j >= n || walls[i][j] {
                    break;
                }
                path_set.insert((i, j));
            }

            if path_set.len() < dirs.len() + 1 {
                continue;
            }

            let mut wall_set = HashSet::new();
            for &(i0, j0) in &path_set {
                if i == i0 && j == j0 {
                    continue;
                }
                for &(di, dj) in &DIRS {
                    let i = i0.wrapping_add_signed(di);
                    let j = j0.wrapping_add_signed(dj);
                    if i < n && j < n && !walls[i][j] && !path_set.contains(&(i, j)) {
                        wall_set.insert((i, j));
                    }
                }
            }
            if !can_add_wall_2(walls, pi, pj, ti, tj, &wall_set) {
                continue;
            }

            {
                let target_i = n - 1;
                let di = if target_i == 0 { 1 } else { -1 };
                let mut wi = target_i.wrapping_add_signed(-di);
                let wj = pj;

                loop {
                    let is_wall = |i: usize, j: usize| walls[i][j] || wall_set.contains(&(i, j));

                    let wi0 = wi;
                    wi = wi.wrapping_add_signed(di);
                    if wi == pi {
                        break;
                    }
                    if is_wall(wi, wj) {
                        continue;
                    }

                    if wi0 < n {
                        if is_wall(wi, wj + 1) && (is_wall(wi0, wj + 1) || is_wall(wi0, wj)) {
                            continue;
                        }
                        if is_wall(wi, wj - 1) && (is_wall(wi0, wj - 1) || is_wall(wi0, wj)) {
                            continue;
                        }
                    }

                    if can_add_wall_2(walls, pi, pj, wi, wj, &wall_set) {
                        wall_set.insert((wi, wj));
                    }
                }
            }

            return Some((wall_set.iter().cloned().collect_vec(), (i, j)));
        }
    }

    None
}

// 縦方向の壁を作る
fn get_initial_vertical_walls(
    walls: &[Vec<bool>],
    pi: usize,
    pj: usize,
    ti: usize,
    tj: usize,
) -> Option<((Vec<(usize, usize)>, (usize, usize)), usize)> {
    let n = walls.len();
    let special = tj < 2 || tj >= n - 2;
    if special && !(ti < 2 || ti >= n - 2) {
        return None; // 縦に伸ばすと入り口がスタート側を向いてしまうため基本的には不採用
    }

    let target_i = if ti > n / 2 { n - 1 } else { 0 };
    let mut prev = vec![vec![None; n]; n];
    let mut queue = vec![(ti, tj, false, false)];

    // 片方向にだけ進んで端を目指す
    while let Some((i, j, i_moved, j_moved)) = queue.pop() {
        if i == target_i && i_moved {
            // 妥当なパスかを確認する
            let (xi, xj) = (i, j);
            let mut path = vec![(i, j)];
            let mut path_set = HashSet::new();
            path_set.insert((i, j));
            loop {
                let Some(&(i, j)) = path.last() else {
                    unreachable!()
                };
                if let Some((i, j)) = prev[i][j] {
                    path.push((i, j));
                    path_set.insert((i, j));
                } else {
                    break;
                }
            }

            let mut wall_set = HashSet::new();
            for &(i, j) in &path {
                for &(di, dj) in &DIRS {
                    let i = i.wrapping_add_signed(di);
                    let j = j.wrapping_add_signed(dj);
                    if i < n && j < n && !walls[i][j] && !path_set.contains(&(i, j)) {
                        wall_set.insert((i, j));
                    }
                }
            }

            // スタートから離れた方向を開けたい
            let di = if (i < n / 2) ^ special { -1 } else { 1 };
            let dj = if j < n / 2 { -1 } else { 1 };

            let xj = xj.wrapping_add_signed(dj);
            if !(xi < n && xj < n && !walls[xi][xj]) {
                continue;
            }
            if !can_reach_goal(walls, pi, pj, xi, xj, &wall_set) {
                continue;
            }
            wall_set.remove(&(xi, xj));
            reduce_wall_set(walls, &mut wall_set, (pi, pj), (ti, tj));
            fill_wall(walls, pi, pj, &mut wall_set);

            // 逆方向に壁を追加する
            let special = special || (tj as isize - xj as isize) * (pj as isize - xj as isize) >= 0;
            if !special {
                let target_i = (n - 1) - target_i;
                let di = if target_i == 0 { 1 } else { -1 };
                let mut wi = target_i.wrapping_add_signed(-di);
                let wj = tj;

                loop {
                    let is_wall = |i: usize, j: usize| walls[i][j] || wall_set.contains(&(i, j));

                    let wi0 = wi;
                    wi = wi.wrapping_add_signed(di);
                    if wi == ti {
                        break;
                    }
                    if is_wall(wi, wj) {
                        continue;
                    }

                    if wi0 < n {
                        if is_wall(wi, wj + 1) && (is_wall(wi0, wj + 1) || is_wall(wi0, wj)) {
                            continue;
                        }
                        if is_wall(wi, wj - 1) && (is_wall(wi0, wj - 1) || is_wall(wi0, wj)) {
                            continue;
                        }
                    }

                    if can_add_wall_2(walls, pi, pj, wi, wj, &wall_set) {
                        wall_set.insert((wi, wj));
                    }
                }
            }

            if special {
                modify_wall_set(walls, &mut wall_set, (pi, pj), (ti, tj), (0, -dj));
            } else {
                modify_wall_set(walls, &mut wall_set, (pi, pj), (ti, tj), (di, 0));
            }

            let level = 0;
            return Some(((wall_set.iter().cloned().collect_vec(), (xi, xj)), level));
        }

        if (i == 1 || i == n - 2) && (j == 0 || j == n - 1) && i_moved && j_moved {
            // 妥当なパスかを確認する
            let (xi, xj) = (i, j);
            let mut path = vec![(i, j)];
            let mut path_set = HashSet::new();
            path_set.insert((i, j));
            loop {
                let Some(&(i, j)) = path.last() else {
                    unreachable!()
                };
                if let Some((i, j)) = prev[i][j] {
                    path.push((i, j));
                    path_set.insert((i, j));
                } else {
                    break;
                }
            }

            let mut wall_set = HashSet::new();
            for &(i, j) in &path {
                for &(di, dj) in &DIRS {
                    let i = i.wrapping_add_signed(di);
                    let j = j.wrapping_add_signed(dj);
                    if i < n && j < n && !walls[i][j] && !path_set.contains(&(i, j)) {
                        wall_set.insert((i, j));
                    }
                }
            }

            // スタートに近い方向を開ける
            let di = if i < n / 2 { 1 } else { -1 };

            let xi = xi.wrapping_add_signed(di);
            if !(xi < n && xj < n && !walls[xi][xj]) {
                continue;
            }
            if !can_reach_goal(walls, pi, pj, xi, xj, &wall_set) {
                continue;
            }
            wall_set.remove(&(xi, xj));
            reduce_wall_set(walls, &mut wall_set, (pi, pj), (ti, tj));
            fill_wall(walls, pi, pj, &mut wall_set);

            modify_wall_set(walls, &mut wall_set, (pi, pj), (ti, tj), (-di, 0));

            let level = 2;
            return Some(((wall_set.iter().cloned().collect_vec(), (xi, xj)), level));
        }

        // 縦方向に移動するまでは縦方向の移動優先度は低い
        if !j_moved {
            if i < target_i && !walls[i + 1][j] && prev[i + 1][j] == None && (i + 1, j) != (ti, tj)
            {
                prev[i + 1][j] = Some((i, j));
                queue.push((i + 1, j, true, j_moved));
            } else if i > target_i
                && !walls[i - 1][j]
                && prev[i - 1][j] == None
                && (i - 1, j) != (ti, tj)
            {
                prev[i - 1][j] = Some((i, j));
                queue.push((i - 1, j, true, j_moved));
            }
        }

        if j < n / 2 {
            if j > 0 && !walls[i][j - 1] && prev[i][j - 1] == None && (i, j - 1) != (ti, tj) {
                prev[i][j - 1] = Some((i, j));
                queue.push((i, j - 1, i_moved, true));
            }
            if j < n - 1 && !walls[i][j + 1] && prev[i][j + 1] == None && (i, j + 1) != (ti, tj) {
                prev[i][j + 1] = Some((i, j));
                queue.push((i, j + 1, i_moved, true));
            }
        } else {
            if j < n - 1 && !walls[i][j + 1] && prev[i][j + 1] == None && (i, j + 1) != (ti, tj) {
                prev[i][j + 1] = Some((i, j));
                queue.push((i, j + 1, i_moved, true));
            }
            if j > 0 && !walls[i][j - 1] && prev[i][j - 1] == None && (i, j - 1) != (ti, tj) {
                prev[i][j - 1] = Some((i, j));
                queue.push((i, j - 1, i_moved, true));
            }
        }

        if j_moved {
            if i < target_i && !walls[i + 1][j] && prev[i + 1][j] == None && (i + 1, j) != (ti, tj)
            {
                prev[i + 1][j] = Some((i, j));
                queue.push((i + 1, j, true, j_moved));
            } else if i > target_i
                && !walls[i - 1][j]
                && prev[i - 1][j] == None
                && (i - 1, j) != (ti, tj)
            {
                prev[i - 1][j] = Some((i, j));
                queue.push((i - 1, j, true, j_moved));
            }
        }

        // 壁に接していた場合、壁から離れる方向の移動は最優先
        if !i_moved && i == target_i {
            if i == 0 && !walls[i + 1][j] && prev[i + 1][j] == None {
                prev[i + 1][j] = Some((i, j));
                queue.push((i + 1, j, true, j_moved));
            } else if i == n - 1 && !walls[i - 1][j] && prev[i - 1][j] == None {
                prev[i - 1][j] = Some((i, j));
                queue.push((i - 1, j, true, j_moved));
            }
        }
    }

    None
}

// 横方向の壁を作る
fn get_initial_horizontal_walls(
    walls: &[Vec<bool>],
    pi: usize,
    pj: usize,
    ti: usize,
    tj: usize,
) -> Option<((Vec<(usize, usize)>, (usize, usize)), usize)> {
    let n = walls.len();
    let special = ti < 2 || ti >= n - 2;
    if special && !(tj < 2 || tj >= n - 2) {
        return None; // 横に伸ばすと入り口がスタート側を向いてしまうため基本的には不採用
    }

    let target_j = if tj > n / 2 { n - 1 } else { 0 };
    let mut prev = vec![vec![None; n]; n];
    let mut queue = vec![(ti, tj, false, false)];

    // 片方向にだけ進んで端を目指す
    while let Some((i, j, i_moved, j_moved)) = queue.pop() {
        if j == target_j && j_moved {
            // 妥当なパスかを確認する
            let (xi, xj) = (i, j);
            let mut path = vec![(i, j)];
            let mut path_set = HashSet::new();
            path_set.insert((i, j));
            loop {
                let Some(&(i, j)) = path.last() else {
                    unreachable!()
                };
                if let Some((i, j)) = prev[i][j] {
                    path.push((i, j));
                    path_set.insert((i, j));
                } else {
                    break;
                }
            }

            let mut wall_set = HashSet::new();
            for &(i, j) in &path {
                for &(di, dj) in &DIRS {
                    let i = i.wrapping_add_signed(di);
                    let j = j.wrapping_add_signed(dj);
                    if i < n && j < n && !walls[i][j] && !path_set.contains(&(i, j)) {
                        wall_set.insert((i, j));
                    }
                }
            }

            // スタートから離れた方向を開けたい
            let di = 1; // 常に下側
            let dj = if (j < n / 2) ^ special { -1 } else { 1 };

            let xi = xi.wrapping_add_signed(di);
            if !(xi < n && xj < n && !walls[xi][xj]) {
                continue;
            }
            if !can_reach_goal(walls, pi, pj, xi, xj, &wall_set) {
                continue;
            }
            wall_set.remove(&(xi, xj));
            reduce_wall_set(walls, &mut wall_set, (pi, pj), (ti, tj));
            fill_wall(walls, pi, pj, &mut wall_set);

            // 逆方向に壁を追加する
            if !special {
                let target_j = (n - 1) - target_j;
                let dj = if target_j == 0 { 1 } else { -1 };
                let mut wj = target_j.wrapping_add_signed(-dj);
                let wi = ti;

                loop {
                    let is_wall = |i: usize, j: usize| walls[i][j] || wall_set.contains(&(i, j));

                    let wj0 = wj;
                    wj = wj.wrapping_add_signed(dj);
                    if wj == tj {
                        break;
                    }
                    if is_wall(wi, wj) {
                        continue;
                    }

                    if wj0 < n {
                        if is_wall(wi + 1, wj) && (is_wall(wi + 1, wj0) || is_wall(wi, wj0)) {
                            continue;
                        }
                        if is_wall(wi - 1, wj) && (is_wall(wi - 1, wj0) || is_wall(wi, wj0)) {
                            continue;
                        }
                    }

                    if can_add_wall_2(walls, pi, pj, wi, wj, &wall_set) {
                        wall_set.insert((wi, wj));
                    }
                }
            }

            if special {
                modify_wall_set(walls, &mut wall_set, (pi, pj), (ti, tj), (-di, 0));
            } else {
                modify_wall_set(walls, &mut wall_set, (pi, pj), (ti, tj), (0, dj));
            }

            let level = 0;
            return Some(((wall_set.iter().cloned().collect_vec(), (xi, xj)), level));
        }

        if (i == 0 || i == n - 1) && (j == 1 || j == n - 2) && i_moved && j_moved {
            // 妥当なパスかを確認する
            let (xi, xj) = (i, j);
            let mut path = vec![(i, j)];
            let mut path_set = HashSet::new();
            path_set.insert((i, j));
            loop {
                let Some(&(i, j)) = path.last() else {
                    unreachable!()
                };
                if let Some((i, j)) = prev[i][j] {
                    path.push((i, j));
                    path_set.insert((i, j));
                } else {
                    break;
                }
            }

            let mut wall_set = HashSet::new();
            for &(i, j) in &path {
                for &(di, dj) in &DIRS {
                    let i = i.wrapping_add_signed(di);
                    let j = j.wrapping_add_signed(dj);
                    if i < n && j < n && !walls[i][j] && !path_set.contains(&(i, j)) {
                        wall_set.insert((i, j));
                    }
                }
            }

            // スタートに近い方向を開ける
            let dj = if j < n / 2 { 1 } else { -1 };

            let xj = xj.wrapping_add_signed(dj);
            if !(xi < n && xj < n && !walls[xi][xj]) {
                continue;
            }
            if !can_reach_goal(walls, pi, pj, xi, xj, &wall_set) {
                continue;
            }
            wall_set.remove(&(xi, xj));
            reduce_wall_set(walls, &mut wall_set, (pi, pj), (ti, tj));
            fill_wall(walls, pi, pj, &mut wall_set);

            modify_wall_set(walls, &mut wall_set, (pi, pj), (ti, tj), (0, -dj));

            let level = 2;
            return Some(((wall_set.iter().cloned().collect_vec(), (xi, xj)), level));
        }

        // 縦方向に移動するまでは横方向の移動優先度は低い
        if !i_moved {
            if j < target_j && !walls[i][j + 1] && prev[i][j + 1] == None && (i, j + 1) != (ti, tj)
            {
                prev[i][j + 1] = Some((i, j));
                queue.push((i, j + 1, i_moved, true));
            } else if j > target_j
                && !walls[i][j - 1]
                && prev[i][j - 1] == None
                && (i, j - 1) != (ti, tj)
            {
                prev[i][j - 1] = Some((i, j));
                queue.push((i, j - 1, i_moved, true));
            }
        }

        if i < n / 2 {
            if i > 0 && !walls[i - 1][j] && prev[i - 1][j] == None && (i - 1, j) != (ti, tj) {
                prev[i - 1][j] = Some((i, j));
                queue.push((i - 1, j, true, j_moved));
            }
            if i < n - 1 && !walls[i + 1][j] && prev[i + 1][j] == None && (i + 1, j) != (ti, tj) {
                prev[i + 1][j] = Some((i, j));
                queue.push((i + 1, j, true, j_moved));
            }
        } else {
            if i < n - 1 && !walls[i + 1][j] && prev[i + 1][j] == None && (i + 1, j) != (ti, tj) {
                prev[i + 1][j] = Some((i, j));
                queue.push((i + 1, j, true, j_moved));
            }
            if i > 0 && !walls[i - 1][j] && prev[i - 1][j] == None && (i - 1, j) != (ti, tj) {
                prev[i - 1][j] = Some((i, j));
                queue.push((i - 1, j, true, j_moved));
            }
        }

        if i_moved {
            if j < target_j && !walls[i][j + 1] && prev[i][j + 1] == None && (i, j + 1) != (ti, tj)
            {
                prev[i][j + 1] = Some((i, j));
                queue.push((i, j + 1, i_moved, true));
            } else if j > target_j
                && !walls[i][j - 1]
                && prev[i][j - 1] == None
                && (i, j - 1) != (ti, tj)
            {
                prev[i][j - 1] = Some((i, j));
                queue.push((i, j - 1, i_moved, true));
            }
        }

        // 壁に接していた場合、壁から離れる方向の移動は最優先
        if !j_moved && j == target_j {
            if j == 0 && !walls[i][j + 1] && prev[i][j + 1] == None {
                prev[i][j + 1] = Some((i, j));
                queue.push((i, j + 1, i_moved, true));
            } else if j == n - 1 && !walls[i][j - 1] && prev[i][j - 1] == None {
                prev[i][j - 1] = Some((i, j));
                queue.push((i, j - 1, i_moved, true));
            }
        }
    }

    None
}

fn get_initial_walls(
    walls: &[Vec<bool>],
    pi: usize,
    pj: usize,
    ti: usize,
    tj: usize,
) -> (Vec<(usize, usize)>, (usize, usize)) {
    if let Some(ret) = get_initial_spiral_walls(walls, pi, pj, ti, tj) {
        return ret;
    }
    if let Some((ret0, level0)) = get_initial_vertical_walls(walls, pi, pj, ti, tj) {
        if let Some((ret1, level1)) = get_initial_horizontal_walls(walls, pi, pj, ti, tj) {
            if level0 > level1 || (level0 == level1 && ret0.0.len() > ret1.0.len()) {
                // レベルは小さい方が優先
                // レベルが同じ場合、追加する壁が多すぎるときだけ横方向を採用する。袋小路かもしれない
                return ret1;
            }
        }
        return ret0;
    }
    if let Some((ret1, _)) = get_initial_horizontal_walls(walls, pi, pj, ti, tj) {
        return ret1;
    }

    (vec![], (ti, tj))
}

fn main() {
    input_interactive! {
        n: usize,
        ti: usize,
        tj: usize,
        b: [Chars; n],
    }

    let q = if cfg!(debug_assertions) {
        input_interactive! {
            q: [(usize, usize); n * n - 1],
        }
        eprintln!("----------------");
        q
    } else {
        vec![]
    };

    let mut confirmed = vec![vec![false; n]; n]; // 確認済み
    let mut walls = vec![vec![false; n]; n]; // 壁 (= 木, トロント)
    for i in 0..n {
        for j in 0..n {
            if b[i][j] == 'T' {
                walls[i][j] = true;
            }
        }
    }

    let mut score = 0usize;
    let mut xi = ti;
    let mut xj = tj;
    let mut prev_pi = usize::MAX;
    let mut prev_pj = usize::MAX;
    let mut qk = 0;
    let mut paths = vec![];
    let mut first_time = vec![vec![true; n]; n];
    loop {
        let (pi, pj, xy) = if cfg!(debug_assertions) {
            let (pi, pj, xy, qk0) = if prev_pi == usize::MAX {
                (0, n / 2, vec![(0, n / 2)], 0)
            } else {
                get_next_state(&confirmed, &walls, prev_pi, prev_pj, ti, tj, &q, qk)
            };
            prev_pi = pi;
            prev_pj = pj;
            paths.push((pi, pj));
            qk = qk0;
            (pi, pj, xy)
        } else {
            input_interactive! {
                pi: usize,
                pj: usize,
                n0: usize,
                xy: [(usize, usize); n0],
            }
            (pi, pj, xy)
        };

        for (x, y) in xy {
            confirmed[x][y] = true;
        }
        if pi == ti && pj == tj {
            break;
        }

        let mut coming_walls = vec![0];

        // トレント新着
        // if pi == 6 && pj == 2 {
        //     eprintln!();
        // }

        if first_time[pi][pj] {
            first_time[pi][pj] = false;
            let dirs = get_dirs_with_priority(pi, pj, xi, xj);
            for &dir in &dirs {
                let (di, dj) = DIRS[dir];
                let mut i = pi;
                let mut j = pj;
                loop {
                    i = i.wrapping_add_signed(di);
                    j = j.wrapping_add_signed(dj);
                    if i >= n || j >= n {
                        break;
                    }
                    if walls[i][j] {
                        break;
                    }
                    if confirmed[i][j] {
                        continue;
                    }

                    // そこに壁を置いても到達可能か確認する
                    if (i, j) == (ti, tj) || !can_add_wall(&walls, pi, pj, i, j) {
                        continue;
                    }
                    coming_walls.push(i);
                    coming_walls.push(j);
                    walls[i][j] = true;
                    break;
                }
            }
        }

        // ゴールにたどり着きにくいように壁を作る
        if score == 0 {
            let (coming_initial_walls, xij) = get_initial_walls(&walls, pi, pj, ti, tj);
            (xi, xj) = xij;
            for &(i, j) in &coming_initial_walls {
                coming_walls.push(i);
                coming_walls.push(j);
                walls[i][j] = true;
            }
        }

        coming_walls[0] = coming_walls.len() / 2;
        println!("{}", coming_walls.iter().join(" "));

        score += 1;
    }

    if cfg!(debug_assertions) {
        eprintln!("----------------");
        // for &(i, j) in &paths {
        //     eprintln!("{i} {j}");
        // }
        eprintln!("{score}");
    }
}
