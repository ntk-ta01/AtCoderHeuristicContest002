use proconio::input;

const DIJ: [(usize, usize); 4] = [(1, 0), (!0, 0), (0, 1), (0, !0)];
const DIR: [char; 4] = ['D', 'U', 'R', 'L'];

const N: usize = 50;
const TIMELIMIT: f64 = 1.8;

type Output = String;

struct Input {
    s: (usize, usize),
    tiles: Vec<Vec<usize>>,
    ps: Vec<Vec<i32>>,
}

fn parse_input() -> Input {
    input! {
        s: (usize, usize),
        tiles: [[usize; N]; N],
        ps: [[i32; N]; N],
    }
    Input { s, tiles, ps }
}

fn parse_output(out: &[usize]) -> Output {
    out.iter().map(|i| DIR[*i]).collect()
}

fn main() {
    let mut timer = Timer::new();
    let input = parse_input();
    let mut out = vec![];
    let mut best_out = vec![];
    let mut visited = vec![vec![false; N]; N];
    let mut visited_tile_num = vec![
        false;
        *input
            .tiles
            .iter()
            .map(|itr| itr.iter().max().unwrap())
            .max()
            .unwrap()
            + 1
    ];
    visited[input.s.0][input.s.1] = true;
    visited_tile_num[input.tiles[input.s.0][input.s.1]] = true;
    dfs(
        (input.s.0, input.s.1),
        &input,
        &mut out,
        &mut best_out,
        &mut visited,
        &mut visited_tile_num,
        &mut timer,
    );
    let output = parse_output(&best_out);
    println!("{}", output);
    let (score, err, _, _) = compute_score_detail(&input, &output);
    eprintln!("{} {}", score, err);
}

fn dfs(
    v: (usize, usize),
    input: &Input,
    out: &mut Vec<usize>,
    best_out: &mut Vec<usize>,
    visited: &mut [Vec<bool>],
    visited_tile: &mut [bool],
    timer: &mut Timer,
) {
    let (h, w) = v;
    for (i, &(dh, dw)) in DIJ.iter().enumerate() {
        let nh = h + dh;
        let nw = w + dw;
        if N <= nh || N <= nw || visited[nh][nw] {
            continue;
        }
        if visited_tile[input.tiles[nh][nw]] {
            // 訪れたことがあるタイル番号は踏めない
            continue;
        }
        visited[nh][nw] = true;
        visited_tile[input.tiles[nh][nw]] = true;
        if input.tiles[h][w] != input.tiles[nh][nw] {
            out.push(i);
        }
        dfs((nh, nw), input, out, best_out, visited, visited_tile, timer);
        if best_out.len() < out.len() {
            *best_out = out.clone();
        }
        if TIMELIMIT < timer.get_time() {
            return;
        }
        if input.tiles[h][w] != input.tiles[nh][nw] {
            out.pop();
        }
        visited_tile[input.tiles[nh][nw]] = false;
        visited[nh][nw] = false;
    }
}

fn compute_score_detail(
    input: &Input,
    out: &Output,
) -> (i32, String, Vec<usize>, Vec<(usize, usize)>) {
    let mut used = vec![0; N * N];
    let (mut i, mut j) = input.s;
    used[input.tiles[i][j]] = 1;
    let mut score = input.ps[i][j];
    let mut steps = vec![(i, j)];
    let mut err = String::new();
    for c in out.chars() {
        let (di, dj) = match c {
            'L' => (0, !0),
            'R' => (0, 1),
            'U' => (!0, 0),
            'D' => (1, 0),
            _ => {
                return (0, "Illegal output".to_owned(), used, steps);
            }
        };
        i += di;
        j += dj;
        if i >= N || j >= N {
            return (0, "Out of range".to_owned(), used, steps);
        }
        steps.push((i, j));
        if used[input.tiles[i][j]] != 0 {
            err = "Stepped on the same tile twice".to_owned();
        }
        used[input.tiles[i][j]] += 1;
        score += input.ps[i][j];
    }
    if !err.is_empty() {
        score = 0;
    }
    (score, err, used, steps)
}

pub fn get_time() -> f64 {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9
}

struct Timer {
    start_time: f64,
}

impl Timer {
    fn new() -> Timer {
        Timer {
            start_time: get_time(),
        }
    }

    fn get_time(&self) -> f64 {
        get_time() - self.start_time
    }
}
