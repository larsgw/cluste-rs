#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]

use crate::hyper_rectangle::HyperRectangle;
use crate::point::{get_range, Point};
use crate::quickselect::median;

#[derive(PartialEq, Debug)]
pub struct Tree<const M: usize> {
    /// hyper-rectangle boundaries
    h: HyperRectangle<M>,

    /// node information
    node: Box<Node<M>>
}

#[derive(PartialEq, Debug)]
pub enum Node<const M: usize> {
    NonLeaf(NonLeaf<M>),
    Leaf(Point<M>)
}

#[derive(PartialEq, Debug)]
pub struct NonLeaf<const M: usize> {
    /// split dimension
    d: usize,
    /// split value
    v: f64,

    /// left child
    l: Tree<M>,
    /// right child
    r: Tree<M>
}

impl<const M: usize> Tree<M> {
    pub fn initialize(points: &[Point<M>]) -> Self {
        let (min, max) = get_range(points);
        let h = HyperRectangle(min, max);
        let d = 0;

        Self::make_node(points, h, d)
    }

    pub fn make_node(points: &[Point<M>], h: HyperRectangle<M>, d: usize) -> Self {
        let node = if points.len() == 1 {
            Node::Leaf(points[0])
        } else {
            let v = median(points, d);
            let (l, r) = Self::split_points(points, &h, d, v);
            Node::NonLeaf(NonLeaf { d, v, l, r})
        };

        Self { h, node: Box::new(node) }
    }

    pub fn split_points(points: &[Point<M>], h: &HyperRectangle<M>, d: usize, v: f64) -> (Self, Self) {
        let new_d = (d + 1) % M;
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
    use crate::hyper_rectangle::HyperRectangle;
    use crate::point::Point;
    use super::*;

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
