use proconio::input;

const N: usize = 50;

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
    let input = parse_input();
}
