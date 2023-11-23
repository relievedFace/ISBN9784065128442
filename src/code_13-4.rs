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
        s: usize,
        t: usize,
        ab: [(usize, usize); m],
    }

    let mut g = vec![vec![]; n];

    for (a, b) in ab.iter() {
        g[*a].push(*b);
        g[*b].push(*a);
    }

    let mut seen = vec![false; n];

    dfs(&g, s, &mut seen);

    if seen[t] {
        println!("Yes");
    } else {
        println!("No");
    }
}
