use std::collections::{BTreeSet, VecDeque};
use std::cmp::min;

#[snippet = "Flow"]
#[allow(dead_code)]
struct Flow {
    // to, capacity, rev
    edges: Vec<Vec<(usize, usize, usize)>>,
}

#[snippet = "Flow"]
impl Flow {
    #[allow(dead_code)]
    fn new(max_size: usize) -> Flow {
        Flow {
            edges: vec![Vec::new(); max_size + 1],
        }
    }

    #[allow(dead_code)]
    fn add_edge(&mut self, from: usize, to: usize, cap: usize) {
        let from_rev = self.edges[to].len();
        let to_rev = self.edges[from].len();

        self.edges[from].push((to, cap, from_rev));
        self.edges[to].push((from, 0, to_rev));
    }

    #[allow(dead_code)]
    fn max_flow_dinic(&mut self, s: usize, t: usize) -> usize {
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

#[snippet = "bipartite_matching"]
pub mod bipartite_matching {
    fn dfs(v: usize, g: &[Vec<usize>], mat: &mut [Option<usize>], used: &mut [bool]) -> bool {
        used[v] = true;
        for &u in &g[v] {
            if mat[u].is_none() || !used[mat[u].unwrap()] && dfs(mat[u].unwrap(), g, mat, used) {
                mat[v] = Some(u);
                mat[u] = Some(v);
                return true;
            }
        }

        false
    }

    pub fn bipartite_matching(g: &[Vec<usize>]) -> usize {
        let mut res = 0;
        let mut mat = vec![None; g.len()];
        for v in 0..g.len() {
            if mat[v].is_none() {
                let mut used = vec![false; g.len()];
                if dfs(v, g, &mut mat, &mut used) {
                    res += 1;
                }
            }
        }
        res
    }
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
