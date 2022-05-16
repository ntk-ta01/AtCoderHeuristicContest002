use proconio::input;

const N: usize = 50;
type Output = String;

const DIJ: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];
const DIR: [char; 4] = ['U', 'D', 'L', 'R'];

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
    let input = parse_input();
    let out = {
        let mut out: Vec<char> = vec![];
        let m = *input
            .tiles
            .iter()
            .map(|line| line.iter().max().unwrap())
            .max()
            .unwrap()
            + 1;
        let mut used_tiles = vec![false; m];
        used_tiles[input.tiles[input.s.0][input.s.1]] = true;
        dfs(&input, &mut out, input.s, &mut used_tiles);
        out.iter().collect::<Output>()
    };
    println!("{}", out);
}

fn dfs(input: &Input, out: &mut Vec<char>, v: (usize, usize), used_tiles: &mut Vec<bool>) {
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
        dfs(input, out, (ni, nj), used_tiles);
        return;
    }
}
