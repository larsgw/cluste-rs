use crate::point::Point;
use crate::hyper_rectangle::HyperRectangle;
use crate::mrkd::{Tree, Node};
use std::ops::Deref;

/// A representation of the set of clusters C used in Section 3 of (Pelleg & Moore, 1999).
///
/// # References
///
/// Pelleg, D., & Moore, A. (1999). Accelerating exact k-means algorithms with geometric reasoning.
///     Proceedings of the Fifth ACM SIGKDD International Conference
///     on Knowledge Discovery and Data Mining, 277–281. <https://doi.org/10.1145/312129.312248>
pub struct Centers<const K: usize, const M: usize> (pub [Point<M>; K]);

impl<const K: usize, const M: usize> Centers<K, M> {
    pub fn new(centers: [Point<M>; K]) -> Self {
        Self(centers)
    }

    /// Update(h, C) as defined in Section 3.1 (p. 280)
    ///
    /// Time complexity: worst case O(r * k * M)
    pub fn update(&self, tree: &Tree<M>) -> ([Point<M>; K], [usize; K]) {
        let mut centers = [(); K].map(|_| Point::<M>::default());
        let mut counts = [0; K];

        match tree.node.deref() {
            // If the node is not a leaf node, check if the hyper-rectangle has an owner
            Node::NonLeaf(node) => {
                match self.owner(&tree.h) {
                    // If it does, update the centers according to the cached info in the node
                    Some(k) => {
                        centers[k] = centers[k] + tree.center_of_mass * tree.number_of_points;
                        counts[k] = counts[k] + tree.number_of_points;
                    },
                    // Else, descend in the child nodes
                    None => {
                        let (centers_l, counts_l) = self.update(&node.l);
                        let (centers_r, counts_r) = self.update(&node.r);
                        for k in 0..K {
                            centers[k] = centers[k] + centers_l[k] + centers_r[k];
                            counts[k] = counts_l[k] + counts_r[k];
                        }
                    }
                };
            },
            // If the node is a leaf node, update the centers as normal
            Node::Leaf(point) => {
                let k = self.closest(point);
                centers[k] = centers[k] + point.clone();
                counts[k] = counts[k] + 1;
            }
        };

        (centers, counts)
    }

    /// Closest center to a point. In this case, no special action is taken when multiple centers
    /// are equally close.
    ///
    /// Time complexity: O(k * M)
    pub fn closest(&self, point: &Point<M>) -> usize {
        let mut min_d = f64::INFINITY;
        let mut min_c = 0;

        for k in 0..K {
            let d = point.distance(&self.0[k]);
            if d < min_d {
                min_d = d;
                min_c = k;
            }
        }

        min_c
    }

    /// owner_C(h) as defined in Section 3, Definition 1 (p. 278)
    ///
    /// Time complexity: O(k * M)
    pub fn owner(&self, h: &HyperRectangle<M>) -> Option<usize> {
        // Find the center closest to the hyper-rectangle. If there are multiple, return early
        let c1 = self.min_d(h)?;

        // Else, check if c1 dominates every other center
        for c2 in 0..K {
            if c1 != c2 && !self.dominates(c1, c2, h) {
                return Option::None;
            }
        }

        Option::Some(c1)
    }

    /// min(d(c, h)) as in Section 3, Theorem 2 (p. 279)
    ///
    /// Time complexity: O(k * M)
    fn min_d(&self, h: &HyperRectangle<M>) -> Option<usize> {
        let mut min_d = f64::INFINITY;
        let mut min_c = 0;
        let mut single_closest = true;

        for c in 0..K {
            let d = h.distance(&self.0[c]);
            if d == min_d {
                single_closest = false;
            } else if d < min_d {
                single_closest = true;
                min_d = d;
                min_c = c;
            }
        }

        if single_closest {
            Option::Some(min_c)
        } else {
            Option::None
        }
    }

    /// domination as defined in Section 3, Definition 3 (p. 279)
    ///
    /// Time complexity: O(M)
    fn dominates(&self, c1: usize, c2: usize, h: &HyperRectangle<M>) -> bool {
        // Find the point p in h that is the furthest in the direction c2 - c1
        let mut p = [0.0; M];
        for d in 0..M {
            p[d] = if self.0[c1].0[d] < self.0[c2].0[d] {
                h.1.0[d]
            } else {
                h.0.0[d]
            };
        }

        // If the distance to that point is shorter from c1 than from c2, c1 dominates c2
        let point = Point(p);
        point.distance(&self.0[c1]) < point.distance(&self.0[c2])
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use super::*;

    #[test]
    fn owner() {
        let h = HyperRectangle(Point([0.0, 0.0]), Point([2.0, 2.0]));
        let centers = Centers::<2, 2>::new([Point([-2.5, -2.5]), Point([3.0, 1.0])]);
        assert_eq!(centers.owner(&h), Option::Some(1));
    }
}
