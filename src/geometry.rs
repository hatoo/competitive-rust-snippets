use cargo_snippet::snippet;
use std;

#[snippet("Vector2D")]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub struct Vector2D(f64, f64);

#[snippet("Vector2D")]
impl Vector2D {
    pub fn add(a: f64, b: f64) -> f64 {
        let c = a + b;
        if c.abs() < 1e-10 {
            0.0
        } else {
            c
        }
    }

    pub fn dot(self, other: Vector2D) -> f64 {
        Self::add(self.0 * other.0, self.1 * other.1)
    }

    pub fn det(self, other: Vector2D) -> f64 {
        Self::add(self.0 * other.1, -self.1 * other.0)
    }
    pub fn len(&self) -> f64 {
        f64::sqrt((self.0).powi(2) + (self.1).powi(2))
    }
    pub fn unit(self) -> Vector2D {
        let l = self.len();
        Vector2D(self.0 / l, self.1 / l)
    }
    pub fn normal(self) -> Vector2D {
        Vector2D(self.1, -self.0)
    }
}

#[snippet("Vector2D")]
impl std::ops::Add for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Vector2D(Vector2D::add(self.0, rhs.0), Vector2D::add(self.1, rhs.1))
    }
}

#[snippet("Vector2D")]
impl std::ops::Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Self::Output {
        Vector2D(Vector2D::add(self.0, -rhs.0), Vector2D::add(self.1, -rhs.1))
    }
}

#[snippet("Vector2D")]
impl std::ops::Mul<f64> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector2D(rhs * self.0, rhs * self.1)
    }
}

#[snippet("Vector2D")]
impl std::ops::Div<f64> for Vector2D {
    type Output = Vector2D;

    fn div(self, rhs: f64) -> Self::Output {
        Vector2D(self.0 / rhs, self.1 / rhs)
    }
}

use crate::total::Total;

#[snippet("convex_hull")]
#[allow(dead_code)]
fn convex_hull(vs: &[Vector2D]) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..vs.len()).collect();
    idx.sort_by_key(|&i| Total((vs[i].0, vs[i].1)));

    let mut res = Vec::new();

    for &i in &idx {
        while res.len() > 1
            && Vector2D::det(
                vs[res[res.len() - 1]] - vs[res[res.len() - 2]],
                vs[i] - vs[res[res.len() - 1]],
            ) <= 0.0
        {
            res.pop();
        }
        res.push(i);
    }
    let t = res.len();

    for &i in idx.iter().rev().skip(1) {
        while res.len() > t
            && (vs[res[res.len() - 1]] - vs[res[res.len() - 2]]).det(vs[i] - vs[res[res.len() - 1]])
                <= 0.0
        {
            res.pop();
        }
        res.push(i);
    }

    res.pop();
    res
}

#[snippet("closest_pair")]
pub fn closest_pair(ps: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
    fn d(p1: (f64, f64), p2: (f64, f64)) -> f64 {
        ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt()
    }

    fn rec(x_sort: &[(f64, f64)], y_sort: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
        if x_sort.len() <= 3 {
            let mut min_d = std::f64::MAX;
            let mut pair = ((0.0, 0.0), (0.0, 0.0));
            for (i, &p1) in x_sort.iter().enumerate() {
                for (j, &p2) in x_sort.iter().enumerate() {
                    if i != j {
                        let dist = d(p1, p2);
                        if dist < min_d {
                            min_d = dist;
                            pair = (p1, p2);
                        }
                    }
                }
            }

            return pair;
        }

        let mid = x_sort.len() / 2;
        let pivot = x_sort[mid].0;

        let q_x = &x_sort[..mid];
        let r_x = &x_sort[mid..];

        let mut q_y = Vec::with_capacity(mid);
        let mut r_y = Vec::with_capacity(x_sort.len() - mid);

        for &(x, y) in y_sort {
            if x < pivot {
                q_y.push((x, y));
            } else {
                r_y.push((x, y));
            }
        }

        let pair1 = rec(q_x, &q_y);
        let pair2 = rec(r_x, &r_y);

        let w = d(pair1.0, pair1.1).min(d(pair2.0, pair2.1));
        let s: Vec<(f64, f64)> = y_sort
            .iter()
            .filter(|&&(x, _)| (pivot - x).abs() <= w)
            .cloned()
            .collect();

        let mut min_d = w;
        let mut pair = if d(pair1.0, pair1.1) < d(pair2.0, pair2.1) {
            pair1
        } else {
            pair2
        };

        for (i, &p1) in s.iter().enumerate() {
            for &p2 in s[i + 1..].iter().take(15) {
                let dist = d(p1, p2);
                if dist < min_d {
                    min_d = dist;
                    pair = (p1, p2);
                }
            }
        }
        pair
    }

    let mut x_sort = ps.to_vec();
    let mut y_sort = ps.to_vec();

    x_sort.sort_by_key(|p| Total(p.0));
    y_sort.sort_by_key(|p| Total(p.1));
    rec(&x_sort, &y_sort)
}

/// Is line a-b and line c-d intersected ?
#[snippet("is_intersected")]
pub fn is_intersected(a: Vector2D, b: Vector2D, c: Vector2D, d: Vector2D) -> bool {
    let ta = (c.0 - d.0) * (a.1 - c.1) + (c.1 - d.1) * (c.0 - a.0);
    let tb = (c.0 - d.0) * (b.1 - c.1) + (c.1 - d.1) * (c.0 - b.0);
    let tc = (a.0 - b.0) * (c.1 - a.1) + (a.1 - b.1) * (a.0 - c.0);
    let td = (a.0 - b.0) * (d.1 - a.1) + (a.1 - b.1) * (a.0 - d.0);

    tc * td <= 0.0 && ta * tb <= 0.0
    // Not intersects start or end point.
    // tc * td < 0.0 && ta * tb < 0.0
}

#[test]
fn test_convex_hull() {
    let vs = vec![
        Vector2D(-1.0, -1.0),
        Vector2D(-1.0, 1.0),
        Vector2D(1.0, 1.0),
        Vector2D(1.0, -1.0),
        Vector2D(0.0, 0.0),
        Vector2D(0.1, 0.1),
    ];

    let mut idx = convex_hull(&vs);
    idx.sort();

    assert_eq!(&idx, &[0, 1, 2, 3]);
}
