use proconio::input;

type Graph = Vec<Vec<usize>>;

fn dfs(g: &Graph, v: usize, seen: &mut Vec<bool>) {
    seen[v] = true;

    for next_v in g[v].iter() {
        if seen[*next_v] {
            continue;
        }

        dfs(g, *next_v, seen);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut g = vec![vec![]; n];

    for (a, b) in ab.iter() {
        g[*a].push(*b);
        g[*b].push(*a);
    }

    let mut seen = vec![false; n];

    for v in 0..n {
        if seen[v] {
            continue;
        }

        dfs(&g, v, &mut seen);
    }
}
