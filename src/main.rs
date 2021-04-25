use proconio::input;
use std::cmp;

// const TIMELIMIT: f64 = 1.955;
fn main() {
    input! {
        si: i32,
        sj: i32,
        ts: [[i32; 50]; 50],
        ps: [[i32; 50]; 50],
    }
    let input = Input { si, sj, ts, ps };

    let state = solve(&input);
    let score = state.state.0;
    let pos = state.state.1;

    println!("{}", pos.route);
    eprintln!("{}", score);
}

fn solve(input: &Input) -> State {
    let nowi = input.si;
    let nowj = input.sj;
    let pos = Position {
        nowi,
        nowj,
        route: String::from(""),
    };

    let beam_width = 500;
    let dij = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut visited = vec![vec![false; 50]; 50];
    visited[nowi as usize][nowj as usize] = true;
    for (di, dj) in dij.iter() {
        if pos.within_range(*di, *dj) {
            let ni = (nowi + di) as usize;
            let nj = (nowj + dj) as usize;
            if input.ts[nowi as usize][nowj as usize] == input.ts[ni][nj] {
                visited[ni][nj] = true;
            }
        }
    }

    let mut states = vec![State {
        state: (0, pos.clone()),
        visited: visited.clone(),
    }];

    let mut best_state = State {
        state: (0, pos.clone()),
        visited,
    };
    for rep in 0..5000 {
        let mut new_states = vec![];
        while !states.is_empty() {
            let now_state = states.pop().unwrap();
            // let now_score = state.state.0;
            for (di, dj) in dij.iter() {
                let mut new_state = now_state.clone();
                // 範囲外でないか、訪れていないかチェック
                let ni = new_state.state.1.nowi + di;
                let nj = new_state.state.1.nowj + dj;
                if new_state.state.1.within_range(*di, *dj)
                    && !new_state.visited[ni as usize][nj as usize]
                {
                    let mut is_move = true; // 進んだ先が同じタイルだったら進んではいけない
                    if *di == -1 {
                        is_move = new_state.state.1.up(&input.ts, &mut new_state.visited);
                    } else if *di == 1 {
                        is_move = new_state.state.1.down(&input.ts, &mut new_state.visited);
                    } else if *dj == -1 {
                        is_move = new_state.state.1.left(&input.ts, &mut new_state.visited);
                    } else if *dj == 1 {
                        is_move = new_state.state.1.right(&input.ts, &mut new_state.visited);
                    }
                    if is_move {
                        let new_route = &new_state.state.1.route;
                        let new_score = compute_score_detail(&input, new_route);
                        // 空白地帯が多い方に加点
                        let add_score = if rep < 10 {
                            20 * compute_space(&input, &new_state)
                        } else {
                            20 * compute_space2(&input, &new_state)
                        };
                        new_state.state.0 = new_score + add_score;
                        if best_state.state.0 < new_score {
                            best_state = new_state.clone();
                        }
                        new_states.push(new_state);
                    }
                }
            }
        }
        states = new_states;
        if beam_width < states.len() {
            states.sort_by(|a, b| a.cmp(b).reverse());
            states = states[..beam_width - (beam_width / 2)].to_vec();
        }
    }
    best_state
}

fn compute_space(_input: &Input, state: &State) -> i32 {
    let dij = vec![(-1, 0), (1, 0), (0, -1), (0, 1)]; // U,D,L,R
    let mut max_space = 0;
    for (di, dj) in dij.iter() {
        let mut now_space = 0;
        let mut pos: Position = state.state.1.clone();
        if *di == -1 {
            while pos.nowi > 0 && !state.visited[(pos.nowi - 1) as usize][pos.nowj as usize]
            // && input.ts[pos.nowi as usize][pos.nowj as usize]
            // != input.ts[pos.nowi as usize - 1][pos.nowj as usize]
            {
                now_space += 1;
                pos.nowi -= 1;
            }
        } else if *di == 1 {
            while pos.nowi < 49 && !state.visited[(pos.nowi + 1) as usize][pos.nowj as usize]
            // && input.ts[pos.nowi as usize][pos.nowj as usize]
            // != input.ts[pos.nowi as usize + 1][pos.nowj as usize]
            {
                now_space += 1;
                pos.nowi += 1;
            }
        } else if *dj == -1 {
            while pos.nowj > 0 && !state.visited[pos.nowi as usize][(pos.nowj - 1) as usize]
            // && input.ts[pos.nowi as usize][pos.nowj as usize - 1]
            // != input.ts[pos.nowi as usize][pos.nowj as usize]
            {
                now_space += 1;
                pos.nowj -= 1;
            }
        } else if *dj == 1 {
            while pos.nowj < 49 && !state.visited[pos.nowi as usize][(pos.nowj + 1) as usize]
            // && input.ts[pos.nowi as usize][pos.nowj as usize + 1]
            // != input.ts[pos.nowi as usize][pos.nowj as usize]
            {
                now_space += 1;
                pos.nowj += 1;
            }
        }
        max_space = max_space.max(now_space);
    }
    max_space
}

fn compute_space2(input: &Input, state: &State) -> i32 {
    let dij = vec![(-1, 0), (1, 0), (0, -1), (0, 1)]; // U,D,L,R
    let mut max_space = 0;
    for (di, dj) in dij.iter() {
        let mut now_space = 0;
        let mut pos: Position = state.state.1.clone();
        if *di == -1 {
            while pos.nowi > 0
                && !state.visited[(pos.nowi - 1) as usize][pos.nowj as usize]
                && input.ts[pos.nowi as usize][pos.nowj as usize]
                    != input.ts[pos.nowi as usize - 1][pos.nowj as usize]
            {
                now_space += 1;
                pos.nowi -= 1;
            }
        } else if *di == 1 {
            while pos.nowi < 49
                && !state.visited[(pos.nowi + 1) as usize][pos.nowj as usize]
                && input.ts[pos.nowi as usize][pos.nowj as usize]
                    != input.ts[pos.nowi as usize + 1][pos.nowj as usize]
            {
                now_space += 1;
                pos.nowi += 1;
            }
        } else if *dj == -1 {
            while pos.nowj > 0
                && !state.visited[pos.nowi as usize][(pos.nowj - 1) as usize]
                && input.ts[pos.nowi as usize][pos.nowj as usize - 1]
                    != input.ts[pos.nowi as usize][pos.nowj as usize]
            {
                now_space += 1;
                pos.nowj -= 1;
            }
        } else if *dj == 1 {
            while pos.nowj < 49
                && !state.visited[pos.nowi as usize][(pos.nowj + 1) as usize]
                && input.ts[pos.nowi as usize][pos.nowj as usize + 1]
                    != input.ts[pos.nowi as usize][pos.nowj as usize]
            {
                now_space += 1;
                pos.nowj += 1;
            }
        }
        max_space = max_space.max(now_space);
    }
    max_space
}

#[allow(dead_code)]
fn compute_score_detail(input: &Input, out: &String) -> i32 {
    let mut used = vec![0; 50 * 50];
    let mut i = input.si as usize;
    let mut j = input.sj as usize;
    used[input.ts[i][j] as usize] = 1;
    let mut score = input.ps[i][j];
    for c in out.chars() {
        let (di, dj) = match c {
            'L' => (0, !0),
            'R' => (0, 1),
            'U' => (!0, 0),
            'D' => (1, 0),
            _ => {
                return 0;
            }
        };
        i += di;
        j += dj;
        if i >= 50 || j >= 50 {
            return 0;
        }
        used[input.ts[i][j] as usize] += 1;
        score += input.ps[i][j];
    }
    score
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Position {
    nowi: i32,
    nowj: i32,
    route: String,
}

impl Position {
    fn up(&mut self, ts: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) -> bool {
        if self.nowi > 0
            && ts[self.nowi as usize][self.nowj as usize]
                != ts[self.nowi as usize - 1][self.nowj as usize]
        {
            self.nowi -= 1;
            self.route.push('U');
            visited[self.nowi as usize][self.nowj as usize] = true;
            // 四方を見て、同じタイル番号があったら訪れたことにする
            let dij = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (di, dj) in dij.iter() {
                if self.within_range(*di, *dj) {
                    let ni = (self.nowi + di) as usize;
                    let nj = (self.nowj + dj) as usize;
                    if ts[self.nowi as usize][self.nowj as usize] == ts[ni][nj] {
                        visited[ni][nj] = true;
                    }
                }
            }
            return true;
        }
        false
    }

    fn down(&mut self, ts: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) -> bool {
        if self.nowi < 49
            && ts[self.nowi as usize][self.nowj as usize]
                != ts[self.nowi as usize + 1][self.nowj as usize]
        {
            self.nowi += 1;
            self.route.push('D');
            visited[self.nowi as usize][self.nowj as usize] = true;
            let dij = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (di, dj) in dij.iter() {
                if self.within_range(*di, *dj) {
                    let ni = (self.nowi + di) as usize;
                    let nj = (self.nowj + dj) as usize;
                    if ts[self.nowi as usize][self.nowj as usize] == ts[ni][nj] {
                        visited[ni][nj] = true;
                    }
                }
            }
            return true;
        }
        false
    }

    fn left(&mut self, ts: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) -> bool {
        if self.nowi > 0
            && ts[self.nowi as usize][self.nowj as usize]
                != ts[self.nowi as usize][self.nowj as usize - 1]
        {
            self.nowj -= 1;
            self.route.push('L');
            visited[self.nowi as usize][self.nowj as usize] = true;
            let dij = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (di, dj) in dij.iter() {
                if self.within_range(*di, *dj) {
                    let ni = (self.nowi + di) as usize;
                    let nj = (self.nowj + dj) as usize;
                    if ts[self.nowi as usize][self.nowj as usize] == ts[ni][nj] {
                        visited[ni][nj] = true;
                    }
                }
            }
            return true;
        }
        false
    }

    fn right(&mut self, ts: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) -> bool {
        if self.nowi < 49
            && ts[self.nowi as usize][self.nowj as usize]
                != ts[self.nowi as usize][self.nowj as usize + 1]
        {
            self.nowj += 1;
            self.route.push('R');
            visited[self.nowi as usize][self.nowj as usize] = true;
            let dij = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (di, dj) in dij.iter() {
                if self.within_range(*di, *dj) {
                    let ni = (self.nowi + di) as usize;
                    let nj = (self.nowj + dj) as usize;
                    if ts[self.nowi as usize][self.nowj as usize] == ts[ni][nj] {
                        visited[ni][nj] = true;
                    }
                }
            }
            return true;
        }
        false
    }

    fn within_range(&self, di: i32, dj: i32) -> bool {
        0 <= self.nowi + di && self.nowi + di < 50 && 0 <= self.nowj + dj && self.nowj + dj < 50
    }
}

#[derive(Clone, Eq)]
struct State {
    state: (i32, Position),
    visited: Vec<Vec<bool>>,
}

impl cmp::Ord for State {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.state.0.cmp(&other.state.0)
    }
}

impl cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.state.0 == other.state.0
    }
}

struct Input {
    si: i32,
    sj: i32,
    ts: Vec<Vec<i32>>,
    ps: Vec<Vec<i32>>,
}
