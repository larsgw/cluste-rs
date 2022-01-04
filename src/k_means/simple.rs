use crate::k_means::centers::Centers;
use crate::mrkd::Tree;
use crate::point::Point;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// Implements the "simple algorithm" for [k-means clustering](https://en.wikipedia.org/wiki/K-means_clustering)
/// (Pelleg & Moore, 1999).
///
/// # References
///
/// Pelleg, D., & Moore, A. (1999). Accelerating exact k-means algorithms with geometric reasoning.
///     Proceedings of the Fifth ACM SIGKDD International Conference
///     on Knowledge Discovery and Data Mining, 277–281. <https://doi.org/10.1145/312129.312248>
pub struct SimpleKMeans<const K: usize, const M: usize, const R: usize> {
    centers: [Point<M>; K],
    point_centers: [usize; R]
}

impl<const K: usize, const M: usize, const R: usize> SimpleKMeans<K, M, R> {
    pub fn fit(points: &[Point<M>; R]) -> Self {
        Self::new(points, Option::None)
    }

    pub fn fit_with_random_state(points: &[Point<M>; R], random_state: u64) -> Self {
        Self::new(points, Option::Some(random_state))
    }

    fn new(points: &[Point<M>; R], random_state: Option<u64>) -> Self {
        // Initialize centers
        let mut rng = match random_state {
            Option::Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy()
        };
        let mut centers = Centers::new(Self::random_points(points, &mut rng));
        let tree = Tree::initialize(points);

        // Update centers
        loop {
            let (new_centers, counts) = centers.update(&tree);

            // For each new center
            let mut different = false;
            for k in 0..K {
                // Finalize updating the centers of mass
                let center = new_centers[k];
                let count = counts[k];
                let new_center = if count == 0 {
                    center
                } else {
                    center / count
                };

                // Check whether convergence is reached
                if centers.0[k] != new_center {
                    different = true;
                }
                centers.0[k] = new_center;
            }


            // If all centers are converged, return
            if !different {
                // Get point centers
                let mut point_centers = [0; R];
                for i in 0..R {
                    point_centers[i] = centers.closest(&points[i]);
                }
                return SimpleKMeans { centers: centers.0, point_centers }
            }
        }
    }

    fn random_points(points: &[Point<M>; R], rng: &mut impl Rng) -> [Point<M>; K] {
        // Ensure initialization so the compiler does not complain
        let mut indices = [0; K];
        // Sample random points to initialize centers
        for (k, i) in rand::seq::index::sample(rng, R, K).iter().enumerate() {
            indices[k] = i;
        }
        indices.map(|i| points[i].clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use super::*;

    #[test]
    fn fit_with_random_state() {
        let points = [
            Point([0.5, 0.5]),
            Point([1.5, 0.5]),
            Point([0.5, 1.5]),
            Point([1.5, 1.5])
        ];
        let SimpleKMeans { centers, point_centers } = SimpleKMeans::<4, 2, 4>::fit_with_random_state(&points, 0);
        assert_eq!(centers, [
            Point([0.5, 0.5]),
            Point([1.5, 1.5]),
            Point([1.5, 0.5]),
            Point([0.5, 1.5])
        ]);
        assert_eq!(point_centers, [0, 2, 3, 1]);
    }
}
