use permutohedron::LexicalPermutation;
use proconio::input;
use std::cmp;

const DIR: [(usize, usize); 4] = [
    (0, 1usize.wrapping_neg()),
    (1, 0),
    (0, 1),
    (1usize.wrapping_neg(), 0),
];
const CHAR: [char; 4] = ['L', 'D', 'R', 'U'];
const N: usize = 50;
const TIMELIMIT: f64 = 1.95;

struct Input {
    si: usize,
    sj: usize,
    ts: Vec<Vec<usize>>,
    ps: Vec<Vec<i32>>,
}
fn main() {
    input! {
        si: usize,
        sj: usize,
        ts: [[usize; 50]; 50],
        ps: [[i32; 50]; 50],
    }
    let input = Input { si, sj, ts, ps };

    let state = solve(&input);

    println!("{}", state.encode());
    eprintln!("{}", state.score);
}

fn dfs(
    ord: &[usize],
    input: &Input,
    state: &mut State,
    best_state: &mut State,
    timer: &mut Timer,
) -> bool {
    if timer.is_over() {
        return false;
    }
    for &oi in ord.iter() {
        let (di, dj) = DIR[oi];
        let nexi = state.pos.i + di;
        let nexj = state.pos.j + dj;
        if nexi > 49 || nexj > 49 {
            continue;
        }
        if state.used[input.ts[nexi][nexj]] {
            continue;
        }
        state.pos.i = nexi;
        state.pos.j = nexj;
        state.score += input.ps[nexi][nexj];
        state.used[input.ts[nexi][nexj]] = true;
        state.route.push(oi);
        if best_state < state {
            *best_state = state.clone();
        }
        if !dfs(ord, input, state, best_state, timer) {
            return false;
        }
        state.pos.i -= di;
        state.pos.j -= dj;
        state.score -= input.ps[nexi][nexj];
        state.used[input.ts[nexi][nexj]] = false;
        state.route.pop();
    }
    true
}

fn solve(input: &Input) -> State {
    let mut timer = Timer::new();

    let mut m = 0;
    for i in 0..N {
        for j in 0..N {
            m = m.max(input.ts[i][j] + 1);
        }
    }

    let mut state = State {
        score: 0,
        pos: Position {
            i: input.si,
            j: input.sj,
        },
        used: vec![false; m],
        route: vec![],
    };
    state.used[input.ts[input.si][input.sj]] = true;
    state.score += input.ps[input.si][input.sj];

    let mut best_state = state.clone();

    let mut ord = [0, 1, 2, 3];

    for _ in 0..24 {
        timer.reset();
        dfs(&ord, input, &mut state.clone(), &mut best_state, &mut timer);
        ord.next_permutation();
    }

    best_state
}

#[allow(dead_code)]
fn compute_score(input: &Input, out: &str) -> i32 {
    let mut i = input.si as usize;
    let mut j = input.sj as usize;
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
        score += input.ps[i][j];
    }
    score
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Position {
    i: usize,
    j: usize,
}

#[derive(Clone, Eq)]
struct State {
    pos: Position,
    score: i32,
    route: Vec<usize>,
    used: Vec<bool>,
}

impl State {
    fn encode(&self) -> String {
        self.route.iter().map(|&idx| CHAR[idx]).collect::<String>()
    }
}

impl cmp::Ord for State {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

pub fn get_time() -> f64 {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9
}

struct Timer {
    start_time: f64,
    time_count: i32,
}

impl Timer {
    fn new() -> Timer {
        Timer {
            start_time: get_time(),
            time_count: 0,
        }
    }

    fn get_time(&self) -> f64 {
        get_time() - self.start_time
    }

    fn reset(&mut self) {
        self.start_time = get_time();
        self.time_count = 0;
    }

    fn is_over(&mut self) -> bool {
        self.time_count += 1;
        if self.time_count >= 100 {
            self.time_count = 0;
            self.get_time() > TIMELIMIT / 24.0
        } else {
            false
        }
    }
}
