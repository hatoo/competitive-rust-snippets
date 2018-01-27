#[snippet = "strongly_connected_component"]
#[allow(dead_code)]
pub fn strongly_connected_component(g: &[Vec<usize>]) -> Vec<usize> {
    fn dfs(v: usize, g: &[Vec<usize>], vs: &mut Vec<usize>, used: &mut [bool]) {
        used[v] = true;
        for &to in &g[v] {
            if !used[to] {
                dfs(to, g, vs, used);
            }
        }
        vs.push(v);
    }
    fn rdfs(v: usize, k: usize, g_rev: &[Vec<usize>], cmp: &mut [usize], used: &mut [bool]) {
        used[v] = true;
        cmp[v] = k;

        for &to in &g_rev[v] {
            if !used[to] {
                rdfs(to, k, g_rev, cmp, used);
            }
        }
    }

    let g_rev = {
        let mut g_rev = vec![Vec::new(); g.len()];
        for (i, es) in g.iter().enumerate() {
            for &to in es {
                g_rev[to].push(i);
            }
        }
        g_rev
    };

    let mut vs = Vec::new();
    let mut used = vec![false; g.len()];
    for v in 0..g.len() {
        if !used[v] {
            dfs(v, g, &mut vs, &mut used);
        }
    }
    let mut used = vec![false; g.len()];
    let mut cmp = vec![0; g.len()];
    let mut k = 0;
    for &v in vs.iter().rev() {
        if !used[v] {
            rdfs(v, k, &g_rev, &mut cmp, &mut used);
            k += 1;
        }
    }

    cmp
}
