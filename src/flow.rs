use cargo_snippet::snippet;
use std::cmp::min;
use std::collections::{BTreeSet, VecDeque};

#[snippet("Flow")]
#[allow(dead_code)]
/// Struct for maximum flow problem
pub struct Flow {
    /// to, capacity, rev
    edges: Vec<Vec<(usize, usize, usize)>>,
}

#[snippet("Flow")]
impl Flow {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Flow {
        Flow {
            edges: vec![Vec::new(); max_size + 1],
        }
    }

    #[allow(dead_code)]
    pub fn add_edge(&mut self, from: usize, to: usize, cap: usize) {
        let from_rev = self.edges[to].len();
        let to_rev = self.edges[from].len();

        self.edges[from].push((to, cap, from_rev));
        self.edges[to].push((from, 0, to_rev));
    }

    #[allow(dead_code)]
    /// Calculate maximum flow by dinic's algorithm
    pub fn max_flow_dinic(&mut self, s: usize, t: usize) -> usize {
        let mut flow = 0;
        loop {
            let level = self.bfs_dinic(s);
            let mut iter = vec![0; self.edges.len()];
            if level[t].is_none() {
                return flow;
            }

            loop {
                let f = self.dfs_dinic(s, t, usize::max_value(), &level, &mut iter);
                if f == 0 {
                    break;
                }
                flow += f;
            }
        }
    }

    #[allow(dead_code)]
    fn bfs_dinic(&self, s: usize) -> Vec<Option<usize>> {
        let mut level = vec![None; self.edges.len()];
        let mut que = VecDeque::new();

        level[s] = Some(0);
        que.push_back(s);

        while !que.is_empty() {
            let v = que.pop_front().unwrap();
            let l = Some(level[v].unwrap() + 1);

            for &(to, cap, _) in &self.edges[v] {
                if cap > 0 && level[to].is_none() {
                    level[to] = l;
                    que.push_back(to);
                }
            }
        }
        level
    }

    #[allow(dead_code)]
    fn dfs_dinic(
        &mut self,
        v: usize,
        t: usize,
        f: usize,
        level: &[Option<usize>],
        iter: &mut [usize],
    ) -> usize {
        if v == t {
            return f;
        }

        for i in iter[v]..self.edges[v].len() {
            iter[v] = i;

            let (to, cap, rev) = self.edges[v][i];
            if cap > 0 && level[v].unwrap() < level[to].unwrap_or(0) {
                let d = self.dfs_dinic(to, t, min(f, cap), level, iter);
                if d > 0 {
                    self.edges[v][i].1 -= d;
                    self.edges[to][rev].1 += d;
                    return d;
                }
            }
        }
        0
    }

    #[allow(dead_code)]
    fn max_flow_ff(&mut self, s: usize, t: usize) -> usize {
        let mut flow = 0;
        let l = self.edges.len();
        loop {
            let f = self.dfs_ff(s, t, usize::max_value(), &mut vec![false; l]);
            if f == 0 {
                break;
            }
            flow += f;
        }
        flow
    }

    #[allow(dead_code)]
    fn dfs_ff(&mut self, v: usize, t: usize, f: usize, used: &mut [bool]) -> usize {
        if v == t {
            return f;
        }
        used[v] = true;

        for i in 0..self.edges[v].len() {
            let (to, cap, rev) = self.edges[v][i];
            if !used[to] && cap > 0 {
                let d = self.dfs_ff(to, t, min(f, cap), used);
                if d > 0 {
                    self.edges[v][i].1 -= d;
                    self.edges[to][rev].1 += d;
                    return d;
                }
            }
        }
        0
    }

    #[allow(dead_code)]
    fn cut(&self, s: usize) -> BTreeSet<usize> {
        let mut stack = Vec::new();
        let mut ss = BTreeSet::new();
        ss.insert(s);
        stack.push(s);

        while !stack.is_empty() {
            let v = stack.pop().unwrap();
            for &(to, cap, _) in &self.edges[v] {
                if cap > 0 && !ss.contains(&to) {
                    ss.insert(to);
                    stack.push(to);
                }
            }
        }

        ss
    }
}

#[snippet("bipartite_matching")]
#[allow(dead_code)]
pub fn bipartite_matching(g: &[Vec<usize>]) -> usize {
    fn dfs(
        v: usize,
        g: &[Vec<usize>],
        mat: &mut [Option<usize>],
        used: &mut [usize],
        id: usize,
    ) -> bool {
        used[v] = id;
        for &u in &g[v] {
            if mat[u].is_none()
                || used[mat[u].unwrap()] != id && dfs(mat[u].unwrap(), g, mat, used, id)
            {
                mat[v] = Some(u);
                mat[u] = Some(v);
                return true;
            }
        }

        false
    }

    let mut res = 0;
    let mut mat = vec![None; g.len()];
    let mut used = vec![0; g.len()];
    for v in 0..g.len() {
        if mat[v].is_none() && dfs(v, g, &mut mat, &mut used, v + 1) {
            res += 1;
        }
    }
    res
}

#[test]
fn test_flow() {
    let mut flow = Flow::new(10);
    let s = 0;
    let t = 4;

    flow.add_edge(s, 1, 10);
    flow.add_edge(s, 2, 2);
    flow.add_edge(1, 2, 6);
    flow.add_edge(1, 3, 6);
    flow.add_edge(3, 2, 3);
    flow.add_edge(2, t, 5);
    flow.add_edge(3, t, 8);

    assert_eq!(flow.max_flow_dinic(s, t), 11);
}

#[test]
fn test_bipartite_matching() {
    use rand::{Rng, SeedableRng, StdRng};
    // Create bipartite graph
    let size = 100;
    let s = 2 * size + 1;
    let t = s + 1;
    let mut flow = Flow::new(2 * size + 2);
    let mut g = vec![Vec::new(); 2 * size];

    for i in 0..size {
        flow.add_edge(s, i, 1);
        flow.add_edge(size + i, t, 1);
    }

    let mut rng = StdRng::from_seed(&[1, 2, 3, 4, 5]);
    for _ in 0..size * size / 4 {
        let u = rng.next_u32() as usize % size;
        let v = size + rng.next_u32() as usize % size;

        if g[u].iter().all(|&x| x != v) {
            g[u].push(v);
            flow.add_edge(u, v, 1);
        }
    }

    assert_eq!(bipartite_matching(&g), flow.max_flow_dinic(s, t));
}
