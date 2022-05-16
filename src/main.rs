use proconio::input;

const N: usize = 50;
type Output = String;

const DIJ: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];
const DIR: [char; 4] = ['U', 'D', 'L', 'R'];

const TIMELIMIT: f64 = 1.5;

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

fn main() {
    let mut timer = Timer::new();
    let input = parse_input();
    let out = {
        let mut best_out = vec![];
        let mut out = vec![];
        let m = *input
            .tiles
            .iter()
            .map(|line| line.iter().max().unwrap())
            .max()
            .unwrap()
            + 1;
        let mut used_tiles = vec![false; m];
        used_tiles[input.tiles[input.s.0][input.s.1]] = true;
        dfs(
            &input,
            &mut out,
            input.s,
            &mut used_tiles,
            &mut timer,
            &mut best_out,
        );
        best_out.iter().collect::<Output>()
    };
    println!("{}", out);
}

fn dfs(
    input: &Input,
    out: &mut Vec<char>,
    v: (usize, usize),
    used_tiles: &mut Vec<bool>,
    timer: &mut Timer,
    best_out: &mut Vec<char>,
) {
    for (i, (di, dj)) in DIJ.iter().enumerate() {
        let ni = v.0 + di;
        let nj = v.1 + dj;
        if ni >= N || nj >= N {
            // 境界チェック
            continue;
        }
        if used_tiles[input.tiles[ni][nj]] {
            continue;
        }
        out.push(DIR[i]);
        used_tiles[input.tiles[ni][nj]] = true;
        dfs(input, out, (ni, nj), used_tiles, timer, best_out);
        if best_out.len() < out.len() {
            *best_out = out.clone();
        }
        if TIMELIMIT < timer.get_time() {
            return;
        }
        out.pop();
        used_tiles[input.tiles[ni][nj]] = false;
    }
}

fn get_time() -> f64 {
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

    // fn reset(&mut self) {
    //     self.start_time = get_time();
    // }
}
