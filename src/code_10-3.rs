use proconio::input;

type Graph = Vec<Vec<usize>>;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut g: Graph = vec![vec![]; n];

    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
}
