import sys
 
Pos = tuple[int, int]
EMPTY = -1
DO_NOTHING = -1
STATION = 0
RAIL_HORIZONTAL = 1
RAIL_VERTICAL = 2
RAIL_LEFT_DOWN = 3
RAIL_LEFT_UP = 4
RAIL_RIGHT_UP = 5
RAIL_RIGHT_DOWN = 6
COST_STATION = 5000
COST_RAIL = 100
 
 
class UnionFind:
    def __init__(self, n: int):
        self.n = n
        self.parents = [-1 for _ in range(n * n)]
 
    def _find_root(self, idx: int) -> int:
        if self.parents[idx] < 0:
            return idx
        self.parents[idx] = self._find_root(self.parents[idx])
        return self.parents[idx]
 
    def is_same(self, p: Pos, q: Pos) -> bool:
        p_idx = p[0] * self.n + p[1]
        q_idx = q[0] * self.n + q[1]
        return self._find_root(p_idx) == self._find_root(q_idx)
 
    def unite(self, p: Pos, q: Pos) -> None:
        p_idx = p[0] * self.n + p[1]
        q_idx = q[0] * self.n + q[1]
        p_root = self._find_root(p_idx)
        q_root = self._find_root(q_idx)
        if p_root != q_root:
            p_size = -self.parents[p_root]
            q_size = -self.parents[q_root]
            if p_size > q_size:
                p_root, q_root = q_root, p_root
            self.parents[q_root] += self.parents[p_root]
            self.parents[p_root] = q_root
 
 
def distance(a: Pos, b: Pos) -> int:
    return abs(a[0] - b[0]) + abs(a[1] - b[1])
 

def is_intersect(a0: Pos, b0: Pos, a1: Pos, b1: Pos) -> bool:
    a2 = [min(a0[0], b0[0]), min(a0[1], b0[1])] 
    b2 = [max(a0[0], b0[0]), max(a0[1], b0[1])] 
    a3 = [min(a1[0], b1[0]), min(a1[1], b1[1])] 
    b3 = [max(a1[0], b1[0]), max(a1[1], b1[1])] 
    if a2[0] > b3[0] or a3[0] > b2[0]:
        return False
    if a2[1] > b3[1] or a3[1] > b2[1]:
        return False
    return True
 

class Action:
    def __init__(self, type: int, pos: Pos):
        self.type = type
        self.pos = pos
 
    def __str__(self):
        if self.type == DO_NOTHING:
            return "-1"
        else:
            return f"{self.type} {self.pos[0]} {self.pos[1]}"
 
 
class Result:
    def __init__(self, actions: list[Action], score: int):
        self.actions = actions
        self.score = score
 
    def __str__(self):
        return "\n".join(map(str, self.actions))
 
 
class Field:
    def __init__(self, N: int):
        self.N = N
        self.rail = [[EMPTY] * N for _ in range(N)]
        self.uf = UnionFind(N)
 
    def build(self, type: int, r: int, c: int) -> None:
        assert self.rail[r][c] != STATION
        if 1 <= type <= 6:
            assert self.rail[r][c] == EMPTY
        self.rail[r][c] = type
 
        # 隣接する区画と接続
        # 上
        if type in (STATION, RAIL_VERTICAL, RAIL_LEFT_UP, RAIL_RIGHT_UP):
            if r > 0 and self.rail[r - 1][c] in (STATION, RAIL_VERTICAL, RAIL_LEFT_DOWN, RAIL_RIGHT_DOWN):
                self.uf.unite((r, c), (r - 1, c))
        # 下
        if type in (STATION, RAIL_VERTICAL, RAIL_LEFT_DOWN, RAIL_RIGHT_DOWN):
            if r < self.N - 1 and self.rail[r + 1][c] in (STATION, RAIL_VERTICAL, RAIL_LEFT_UP, RAIL_RIGHT_UP):
                self.uf.unite((r, c), (r + 1, c))
        # 左
        if type in (STATION, RAIL_HORIZONTAL, RAIL_LEFT_DOWN, RAIL_LEFT_UP):
            if c > 0 and self.rail[r][c - 1] in (STATION, RAIL_HORIZONTAL, RAIL_RIGHT_DOWN, RAIL_RIGHT_UP):
                self.uf.unite((r, c), (r, c - 1))
        # 右
        if type in (STATION, RAIL_HORIZONTAL, RAIL_RIGHT_DOWN, RAIL_RIGHT_UP):
            if c < self.N - 1 and self.rail[r][c + 1] in (STATION, RAIL_HORIZONTAL, RAIL_LEFT_DOWN, RAIL_LEFT_UP):
                self.uf.unite((r, c), (r, c + 1))
 
    def is_connected(self, s: Pos, t: Pos) -> bool:
        assert distance(s, t) > 4  # 前提条件
        stations0 = self.collect_stations(s)
        stations1 = self.collect_stations(t)
        for station0 in stations0:
            for station1 in stations1:
                if self.uf.is_same(station0, station1):
                    return True
        return False
 
    def collect_stations(self, pos: Pos) -> list[Pos]:
        stations = []
        for dr in range(-2, 3):
            for dc in range(-2, 3):
                if abs(dr) + abs(dc) > 2:
                    continue
                r = pos[0] + dr
                c = pos[1] + dc
                if 0 <= r < self.N and 0 <= c < self.N and self.rail[r][c] == STATION:
                    stations.append((r, c))
        return stations
 
 
class Solver:
    def __init__(self, N: int, M: int, K: int, T: int, home: list[Pos], workplace: list[Pos]):
        self.N = N
        self.M = M
        self.K = K
        self.T = T
        self.home = home
        self.workplace = workplace
        self.field = Field(N)
        self.money = K
        self.actions = []
 
    def calc_income(self) -> int:
        income = 0
        for i in range(self.M):
            if self.field.is_connected(self.home[i], self.workplace[i]):
                income += distance(self.home[i], self.workplace[i])
        return income
 
    def build_rail(self, type: int, r: int, c: int) -> None:
        self.field.build(type, r, c)
        self.money -= COST_RAIL
        self.actions.append(Action(type, (r, c)))

    def build_station(self, r: int, c: int) -> None:
        self.field.build(STATION, r, c)
        self.money -= COST_STATION
        self.actions.append(Action(STATION, (r, c)))
 
    def build_nothing(self) -> None:
        self.actions.append(Action(DO_NOTHING, (0, 0)))
 
    def solve(self) -> Result:
        candidates = [True] * self.M
        income = self.calc_income()

        while len(self.actions) < self.T:
            # 接続する人 (条件を満たす範囲で一番長いもの) を見つける
            rail_count = (self.money - COST_STATION * 2) // COST_RAIL

            person_idx = -1
            for i in range(0, self.M):
                if not candidates[i]:
                    continue
                # if self.field.is_connected(self.home[i], self.workplace[i]):
                #     continue
                dist = distance(self.home[i], self.workplace[i])
                if self.money + (dist - 5) * (income - COST_RAIL) - income < 0:
                    continue
                if self.money + (dist - 5) * (income - COST_RAIL) + 2 * (income - COST_STATION) - income < 0:
                    continue
                if (self.T - len(self.actions)) * dist < COST_STATION * 2 + COST_RAIL * (dist - 5):
                    continue # 最後まで残しても赤字になる
                if person_idx >= 0 and dist <= distance(self.home[person_idx], self.workplace[person_idx]):
                    continue
                person_idx = i

            if person_idx == -1:
                if income == 0:
                    while len(self.actions) < self.T:
                        self.build_nothing()
                    break
                self.build_nothing()
                self.money += income
                continue

            # 内側に場所を動かす
            r0, c0 = self.home[person_idx]
            r1, c1 = self.workplace[person_idx]
            if r0 <= r1 - 4:
                r0 += 2
                r1 -= 2
            elif r0 == r1 - 3:
                r0 += 1
                r1 -= 2
                if c0 < c1:
                    c0 += 1
                else:
                    c0 -= 1
            elif r0 == r1 - 2:
                r1 -= 2
                if c0 < c1:
                    c0 += 2
                else:
                    c0 -= 2
            elif r0 == r1 - 1:
                r1 -= 1
                if c0 < c1:
                    c0 += 2
                    c1 -= 1
                else:
                    c0 -= 2
                    c1 += 1
            elif r0 == r1:
                if c0 < c1:
                    c0 += 2
                    c1 -= 2
                else:
                    c0 -= 2
                    c1 += 2
            elif r0 == r1 + 1:
                r1 += 1
                if c0 < c1:
                    c0 += 2
                    c1 -= 1
                else:
                    c0 -= 2
                    c1 += 1
            elif r0 == r1 + 2:
                r1 += 2
                if c0 < c1:
                    c0 += 2
                else:
                    c0 -= 2
            elif r0 == r1 + 3:
                r0 -= 1
                r1 += 2
                if c0 < c1:
                    c0 += 1
                else:
                    c0 -= 1
            else:
                r0 -= 2
                r1 += 2

            # 干渉するものは今後の候補から外す
            candidates[person_idx] = False
            for i in range(0, self.M):
                if candidates[i] and is_intersect([r0, c0], [r1, c1], self.home[i], self.workplace[i]):
                    candidates[i] = False

            # 線路を配置して駅を接続する
            # r0 -> r1
            if r0 < r1:
                for r in range(r0 + 1, r1):
                    self.build_rail(RAIL_VERTICAL, r, c0)
                    self.money += income
                if c0 < c1:
                    self.build_rail(RAIL_RIGHT_UP, r1, c0)
                    self.money += income
                elif c0 > c1:
                    self.build_rail(RAIL_LEFT_UP, r1, c0)
                    self.money += income
            elif r0 > r1:
                for r in range(r0 - 1, r1, -1):
                    self.build_rail(RAIL_VERTICAL, r, c0)
                    self.money += income
                if c0 < c1:
                    self.build_rail(RAIL_RIGHT_DOWN, r1, c0)
                    self.money += income
                elif c0 > c1:
                    self.build_rail(RAIL_LEFT_DOWN, r1, c0)
                    self.money += income
            # c0 -> c1
            if c0 < c1:
                for c in range(c0 + 1, c1):
                    self.build_rail(RAIL_HORIZONTAL, r1, c)
                    self.money += income
            elif c0 > c1:
                for c in range(c0 - 1, c1, -1):
                    self.build_rail(RAIL_HORIZONTAL, r1, c)
                    self.money += income
    
            # 駅の配置
            self.build_station(r0, c0)
            self.build_station(r1, c1)
    
            self.money -= income
            income = self.calc_income()
            self.money += income
 
        return Result(self.actions, self.money)
 
 
def main():
    N, M, K, T = map(int, input().split())
    home = []
    workplace = []
    for _ in range(M):
        r0, c0, r1, c1 = map(int, input().split())
        home.append((r0, c0))
        workplace.append((r1, c1))
 
    solver = Solver(N, M, K, T, home, workplace)
    result = solver.solve()
    print(result)
    print(f"score={result.score}", file=sys.stderr)
 
 
if __name__ == "__main__":
    main()
