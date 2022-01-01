#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]

use crate::Point;
use crate::quickselect::median;

#[derive(PartialEq, Debug)]
pub struct HyperRectangle<const N: usize> (Point<N>, Point<N>);

impl<const N: usize> HyperRectangle<N> {
    fn split(&self, d: usize, v: f64) -> (Self, Self) {
        let mut a = self.1.clone();
        a.0[d] = v;
        let mut b = self.0.clone();
        b.0[d] = v;

        (
            HyperRectangle(self.0, a),
            HyperRectangle(b, self.1),
        )
    }
}

#[derive(PartialEq, Debug)]
pub struct Tree<const N: usize> {
    /// hyper-rectangle boundaries
    h: HyperRectangle<N>,

    /// node information
    node: Box<Node<N>>
}

#[derive(PartialEq, Debug)]
pub enum Node<const N: usize> {
    NonLeaf(NonLeaf<N>),
    Leaf(Point<N>)
}

#[derive(PartialEq, Debug)]
pub struct NonLeaf<const N: usize> {
    /// split dimension
    d: usize,
    /// split value
    v: f64,

    /// left child
    l: Tree<N>,
    /// right child
    r: Tree<N>
}

impl<const N: usize> Tree<N> {
    fn initialize(points: &[Point<N>]) -> Self {
        let mut min = [f64::INFINITY; N];
        let mut max = [f64::NEG_INFINITY; N];

        for point in points {
            for d in 0..N {
                if point.0[d] < min[d] { min[d] = point.0[d]; }
                if point.0[d] > max[d] { max[d] = point.0[d]; }
            }
        }

        let h = HyperRectangle(Point(min), Point(max));
        let d = 0;

        Self::make_node(points, h, d)
    }

    fn make_node(points: &[Point<N>], h: HyperRectangle<N>, d: usize) -> Self {
        let node = if points.len() == 1 {
            Node::Leaf(points[0])
        } else {
            let v = median(points, d);
            let (l, r) = Self::split_points(points, &h, d, v);
            Node::NonLeaf(NonLeaf { d, v, l, r})
        };

        Self { h, node: Box::new(node) }
    }

    fn split_points(points: &[Point<N>], h: &HyperRectangle<N>, d: usize, v: f64) -> (Self, Self) {
        let new_d = (d + 1) % N;
        let len = points.len();

        let (h1, h2) = h.split(d, v);
        let mut p1 = Vec::with_capacity(len / 2 + 1);
        let mut p2 = Vec::with_capacity(len / 2 + 1);

        for point in points {
            if point.0[d] <= v {
                p1.push(point.to_owned())
            } else {
                p2.push(point.to_owned())
            }
        }

        (
            Self::make_node(&p1, h1, new_d),
            Self::make_node(&p2, h2, new_d)
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Point;
    use super::*;

    #[test]
    fn hyper_rectangle_split() {
        let h = HyperRectangle(Point([0.0, 0.0]), Point([2.0, 2.0]));
        let (h1, h2) = h.split(1, 1.0);
        assert_eq!(h1, HyperRectangle(Point([0.0, 0.0]), Point([2.0, 1.0])));
        assert_eq!(h2, HyperRectangle(Point([0.0, 1.0]), Point([2.0, 2.0])));
    }

    #[test]
    fn tree_initialize() {
        let points = vec![
            Point([0.5, 0.5]),
            Point([1.5, 0.5]),
            Point([0.5, 1.5]),
            Point([1.5, 1.5])
        ];
        let tree = Tree::initialize(&points);

        assert_eq!(tree, Tree {
            h: HyperRectangle(Point([0.5, 0.5]), Point([1.5, 1.5])),
            node: Box::new(Node::NonLeaf(NonLeaf {
                d: 0,
                v: 0.5,
                l: Tree {
                    h: HyperRectangle(Point([0.5, 0.5]), Point([0.5, 1.5])),
                    node: Box::new(Node::NonLeaf(NonLeaf {
                        d: 1,
                        v: 0.5,
                        l: Tree {
                            h: HyperRectangle(Point([0.5, 0.5]), Point([0.5, 0.5])),
                            node: Box::new(Node::Leaf(Point([0.5, 0.5])))
                        },
                        r: Tree {
                            h: HyperRectangle(Point([0.5, 0.5]), Point([0.5, 1.5])),
                            node: Box::new(Node::Leaf(Point([0.5, 1.5])))
                        }
                    }))
                },
                r: Tree {
                    h: HyperRectangle(Point([0.5, 0.5]), Point([1.5, 1.5])),
                    node: Box::new(Node::NonLeaf(NonLeaf {
                        d: 1,
                        v: 0.5,
                        l: Tree {
                            h: HyperRectangle(Point([0.5, 0.5]), Point([1.5, 0.5])),
                            node: Box::new(Node::Leaf(Point([1.5, 0.5])))
                        },
                        r: Tree {
                            h: HyperRectangle(Point([0.5, 0.5]), Point([1.5, 1.5])),
                            node: Box::new(Node::Leaf(Point([1.5, 1.5])))
                        }
                    }))
                }
            }))
        });
    }
}
