use crate::hyper_rectangle::HyperRectangle;
use crate::point::{get_range, Point};
use crate::quickselect::median;
use rand::Rng;

#[derive(PartialEq, Debug)]
pub struct Tree<const M: usize> {
    /// Hyper-rectangle boundaries
    pub h: HyperRectangle<M>,

    /// Number of points in contained leaf nodes
    pub number_of_points: usize,

    /// Center of mass of contained points
    pub center_of_mass: Point<M>,

    /// Sum of Euclidean norms of contained points
    pub euclidean_norm_sum: f64,

    /// Node information
    pub node: Box<Node<M>>
}

#[derive(PartialEq, Debug)]
pub enum Node<const M: usize> {
    NonLeaf(NonLeaf<M>),
    Leaf(Point<M>)
}

impl<const M: usize> Node<M> {
    pub fn get_points(&self) -> Box<dyn Iterator<Item = &Point<M>> + '_> {
        match self {
            Node::NonLeaf(node) => Box::new(node.l.get_points().chain(node.r.get_points())),
            Node::Leaf(point) => Box::new(std::iter::once(point))
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct NonLeaf<const M: usize> {
    /// split dimension
    d: usize,
    /// split value
    v: f64,

    /// left child
    pub l: Tree<M>,
    /// right child
    pub r: Tree<M>
}

impl<const M: usize> Tree<M> {
    pub fn initialize(points: &[Point<M>], rng: &mut impl Rng) -> Self {
        let (min, max) = get_range(points);
        let h = HyperRectangle(min, max);
        let d = 0;

        Self::make_node(points, h, d, rng)
    }

    fn make_node(points: &[Point<M>], h: HyperRectangle<M>, d: usize, rng: &mut impl Rng) -> Self {
        // Determine the cached information that makes this a mrkd-tree instead of a kd-tree
        let number_of_points = points.len();
        let mut euclidean_norm_sum = 0.0;
        let mut center_of_mass = Point::default();
        for point in points {
            euclidean_norm_sum += point.distance(&Point::default());
            center_of_mass = center_of_mass + *point;
        }
        center_of_mass = center_of_mass / number_of_points;

        let node = if points.len() == 1 {
            // If only one point remains, make a simple leaf node
            Node::Leaf(points[0])
        } else {
            // If more points remain, determine the split value
            let v = median(points, d, rng);
            // And split the points accordingly
            let (l, r) = Self::split_points(points, &h, d, v, rng);
            Node::NonLeaf(NonLeaf { d, v, l, r})
        };

        Self {
            h,
            number_of_points,
            center_of_mass,
            euclidean_norm_sum,
            node: Box::new(node)
        }
    }

    fn split_points(points: &[Point<M>], h: &HyperRectangle<M>, d: usize, v: f64, rng: &mut impl Rng) -> (Self, Self) {
        // Determine the next split dimension
        let new_d = (d + 1) % M;
        let len = points.len();

        // Split the hyper-rectangle
        let (h1, h2) = h.split(d, v);
        let mut p1 = Vec::with_capacity(len / 2 + 1);
        let mut p2 = Vec::with_capacity(len / 2 + 1);

        // Divide the points
        for point in points {
            if point.0[d] <= v {
                p1.push(point.to_owned())
            } else {
                p2.push(point.to_owned())
            }
        }

        // Make nodes for the two new hyper-rectangles
        (
            Self::make_node(&p1, h1, new_d, rng),
            Self::make_node(&p2, h2, new_d, rng)
        )
    }

    pub fn get_points(&self) -> Box<dyn Iterator<Item = &Point<M>> + '_> {
        self.node.get_points()
    }
}

#[cfg(test)]
mod tests {
    use crate::hyper_rectangle::HyperRectangle;
    use crate::point::Point;
    use rand::SeedableRng;
    use rand::rngs::StdRng;
    use super::*;

    #[test]
    fn tree_initialize() {
        let mut rng = StdRng::seed_from_u64(0);
        let points = vec![
            Point([0.5, 0.5]),
            Point([1.5, 0.5]),
            Point([0.5, 1.5]),
            Point([1.5, 1.5])
        ];
        let tree = Tree::initialize(&points, &mut rng);

        assert_eq!(tree, Tree {
            h: HyperRectangle(Point([0.5, 0.5]), Point([1.5, 1.5])),
            number_of_points: 4,
            center_of_mass: Point([1.0, 1.0]),
            euclidean_norm_sum: 5.99070478491457,
            node: Box::new(Node::NonLeaf(NonLeaf {
                d: 0,
                v: 0.5,
                l: Tree {
                    h: HyperRectangle(Point([0.5, 0.5]), Point([0.5, 1.5])),
                    number_of_points: 2,
                    center_of_mass: Point([0.5, 1.0]),
                    euclidean_norm_sum: 2.2882456112707374,
                    node: Box::new(Node::NonLeaf(NonLeaf {
                        d: 1,
                        v: 0.5,
                        l: Tree {
                            h: HyperRectangle(Point([0.5, 0.5]), Point([0.5, 0.5])),
                            number_of_points: 1,
                            center_of_mass: Point([0.5, 0.5]),
                            euclidean_norm_sum: 0.7071067811865476,
                            node: Box::new(Node::Leaf(Point([0.5, 0.5])))
                        },
                        r: Tree {
                            h: HyperRectangle(Point([0.5, 0.5]), Point([0.5, 1.5])),
                            number_of_points: 1,
                            center_of_mass: Point([0.5, 1.5]),
                            euclidean_norm_sum: 1.5811388300841898,
                            node: Box::new(Node::Leaf(Point([0.5, 1.5])))
                        }
                    }))
                },
                r: Tree {
                    h: HyperRectangle(Point([0.5, 0.5]), Point([1.5, 1.5])),
                    number_of_points: 2,
                    center_of_mass: Point([1.5, 1.0]),
                    euclidean_norm_sum: 3.702459173643832,
                    node: Box::new(Node::NonLeaf(NonLeaf {
                        d: 1,
                        v: 0.5,
                        l: Tree {
                            h: HyperRectangle(Point([0.5, 0.5]), Point([1.5, 0.5])),
                            number_of_points: 1,
                            center_of_mass: Point([1.5, 0.5]),
                            euclidean_norm_sum: 1.5811388300841898,
                            node: Box::new(Node::Leaf(Point([1.5, 0.5])))
                        },
                        r: Tree {
                            h: HyperRectangle(Point([0.5, 0.5]), Point([1.5, 1.5])),
                            number_of_points: 1,
                            center_of_mass: Point([1.5, 1.5]),
                            euclidean_norm_sum: 2.1213203435596424,
                            node: Box::new(Node::Leaf(Point([1.5, 1.5])))
                        }
                    }))
                }
            }))
        });
    }
}
