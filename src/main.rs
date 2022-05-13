use proconio::input;
use rand::prelude::*;

const DIJ: [(usize, usize); 4] = [(1, 0), (!0, 0), (0, 1), (0, !0)];

const N: usize = 50;
const TIMELIMIT: f64 = 1.99;

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

fn main() {
    let mut timer = Timer::new();
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(93216000);
    let input = parse_input();
    let output = annealing(&input, &mut timer, &mut rng);
    println!("{}", output);
}

fn annealing(input: &Input, timer: &mut Timer, rng: &mut rand_chacha::ChaCha20Rng) -> Output {
    const T0: f64 = 200.0;
    const T1: f64 = 1.0;
    let mut temp = T0;
    let mut prob;

    let m = *input
        .tiles
        .iter()
        .map(|itr| itr.iter().max().unwrap())
        .max()
        .unwrap()
        + 1;

    let mut path = vec![input.s];

    let (mut used_tile_prev, mut used_pos_prev, mut score_prev) = {
        let mut v1 = vec![m; m];
        let mut v2 = vec![vec![false; N]; N];
        let mut v3 = vec![0];
        for (i, &(y, x)) in path.iter().enumerate() {
            v1[input.tiles[y][x]] = i;
            v2[y][x] = true;
            v3.push(v3[v3.len() - 1] + input.ps[y][x]);
        }
        (v1, v2, v3)
    };

    let mut best_score = score_prev[score_prev.len() - 1];
    let mut best_path = path.clone();

    let mut ord = [0, 1, 2, 3];

    let mut count = 0;
    loop {
        if count >= 100 {
            let passed = timer.get_time() / TIMELIMIT;
            if passed >= 1.0 {
                break;
            }
            temp = T0.powf(1.0 - passed) * T1.powf(passed);
            count = 0;
        }
        count += 1;
        // 近傍解作成
        let start = rng.gen_range(0, path.len());
        let mut used_tile_next = vec![false; m];
        let get_used_tile = |y: usize, x: usize, used_tile_next: &Vec<bool>| -> bool {
            let i = input.tiles[y][x];
            used_tile_prev[i] <= start || used_tile_next[i]
        };
        let mut score_next = score_prev[start + 1];
        let mut diff = vec![];
        let (mut y, mut x) = path[start];
        loop {
            let mut found = false;
            ord.shuffle(rng);
            for &o in ord.iter() {
                let nh = y + DIJ[o].0;
                let nw = x + DIJ[o].1;
                if N <= nh || N <= nw {
                    continue;
                }
                if diff.is_empty() && start + 1 < path.len() && path[start + 1] == (nh, nw) {
                    continue;
                }
                if !get_used_tile(nh, nw, &used_tile_next) {
                    found = true;
                    diff.push((nh, nw));
                    y = nh;
                    x = nw;
                    used_tile_next[input.tiles[y][x]] = true;
                    score_next += input.ps[y][x];
                    break;
                }
            }
            if !found {
                break;
            }
            if used_pos_prev[y][x] {
                break;
            }
        }
        if diff.is_empty() {
            continue;
        }
        let mut tail_first = path.len();
        let mut tail_last = path.len();
        if used_pos_prev[y][x] {
            // 合流できた場合
            tail_first = start;
            while tail_first < path.len() && path[tail_first] != (y, x) {
                tail_first += 1;
            }
            tail_first += 1;
            // 合流後の以前のパスが、新しいパスのタイルを踏んでしまうと合流後の以前のパスを短くしなければならない
            for (i, &(y, x)) in path.iter().enumerate().skip(tail_first) {
                if get_used_tile(y, x, &used_tile_next) {
                    tail_last = i;
                    break;
                }
                used_tile_next[input.tiles[y][x]] = true;
            }
        }
        score_next += score_prev[tail_last] - score_prev[tail_first];
        // 近傍解作成ここまで
        prob = f64::exp((score_next - score_prev[score_prev.len() - 1]) as f64 / temp);
        if score_prev[score_prev.len() - 1] < score_next || rng.gen_bool(prob) {
            // 合流できて解を更新する場合、まずdiffに合流後のpathを足す
            let mut add_diff = path
                .clone()
                .into_iter()
                .skip(tail_first)
                .take(tail_last - tail_first)
                .collect::<Vec<_>>();
            diff.append(&mut add_diff);
            // 次にpathをstart+1の長さまで切り詰める
            while start + 1 < path.len() {
                path.pop();
            }
            // pathの後ろにdiffを挿入する
            path.append(&mut diff);
            // 諸々の更新
            let mut v1 = vec![m; m];
            let mut v2 = vec![vec![false; N]; N];
            let mut v3 = vec![0];
            for (i, &(y, x)) in path.iter().enumerate() {
                v1[input.tiles[y][x]] = i;
                v2[y][x] = true;
                v3.push(v3[v3.len() - 1] + input.ps[y][x]);
            }
            used_tile_prev = v1;
            used_pos_prev = v2;
            score_prev = v3;
        }

        if best_score < score_prev[score_prev.len() - 1] {
            best_score = score_prev[score_prev.len() - 1];
            best_path = path.clone();
        }
    }

    // parse_output
    let mut output = vec![];
    for (&pre, &now) in best_path.iter().zip(best_path.iter().skip(1)) {
        let di = now.0 as i32 - pre.0 as i32;
        let dj = now.1 as i32 - pre.1 as i32;
        let ch = match (di, dj) {
            (0, -1) => 'L',
            (0, 1) => 'R',
            (-1, 0) => 'U',
            (1, 0) => 'D',
            _ => unreachable!(),
        };
        output.push(ch);
    }
    // eprintln!("{}", best_score);
    output.iter().collect()
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

    // fn reset(&mut self) {
    //     self.start_time = get_time();
    // }
}
