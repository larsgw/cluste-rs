use crate::centers::Centers;
use crate::mrkd::Tree;
use crate::point::Point;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// Algorithm used for [k-means clustering](https://en.wikipedia.org/wiki/K-means_clustering).
///
/// # References
///
/// Lloyd, S. (1982). Least squares quantization in PCM. IEEE Transactions on Information Theory,
///     28(2), 129–137. <https://doi.org/10.1109/TIT.1982.1056489>
///
/// Pelleg, D., & Moore, A. (1999). Accelerating exact k-means algorithms with geometric reasoning.
///     Proceedings of the Fifth ACM SIGKDD International Conference
///     on Knowledge Discovery and Data Mining, 277–281. <https://doi.org/10.1145/312129.312248>
#[derive(PartialEq)]
pub enum Algorithm {
    /// Use Lloyd's algorithm (Lloyd, 1982) as described in (Pelleg & Moore, 1999).
    Naive,
    /// Use the "simple" algorithm described in (Pelleg & Moore).
    Simple
}

/// Implements [k-means clustering](https://en.wikipedia.org/wiki/K-means_clustering).
pub struct KMeans<const K: usize, const M: usize, const R: usize> {
    centers: [Point<M>; K],
    point_centers: [usize; R]
}

impl<const K: usize, const M: usize, const R: usize> KMeans<K, M, R> {
    /// Get k clusters based on `points`.
    pub fn fit(points: &[Point<M>; R], algorithm: Algorithm) -> Self {
        Self::new(points, algorithm, Option::None)
    }

    /// Get k clusters based on `points` with a pre-determined random state.
    pub fn fit_with_random_state(points: &[Point<M>; R], algorithm: Algorithm, random_state: u64) -> Self {
        Self::new(points, algorithm, Option::Some(random_state))
    }

    fn new(points: &[Point<M>; R], algorithm: Algorithm, random_state: Option<u64>) -> Self {
        // Initialize randomness
        let mut rng = match random_state {
            Option::Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy()
        };

        // Initialize centers
        let mut centers = Centers::new(Self::random_points(points, &mut rng));

        // Initialize tree when necessary
        let tree = match algorithm {
            Algorithm::Simple => Option::Some(Tree::initialize(points, &mut rng)),
            Algorithm::Naive => Option::None
        };

        // Update centers
        loop {
            let mut point_centers = [0; R];
            let mut new_centers = [Point::default(); K];
            let mut new_counts = [0; K];

            match algorithm {
                Algorithm::Simple => {
                    // Use Update(h, C)
                    let updated = centers.update(&tree.as_ref().unwrap());
                    new_centers = updated.0;
                    new_counts = updated.1;
                },
                Algorithm::Naive => {
                    // For each data point
                    for i in 0..R {
                        // Find the closest center
                        let mut min_d = f64::INFINITY;
                        let mut min_c = 0;
                        for k in 0..K {
                            let d = centers.0[k].distance(&points[i]);
                            if d < min_d {
                                min_d = d;
                                min_c = k;
                            }
                        }

                        // Update the center associated with the data point
                        point_centers[i] = min_c;

                        // Update the center of mass
                        new_centers[min_c] = new_centers[min_c] + points[i];
                        new_counts[min_c] = new_counts[min_c] + 1;
                    }
                }
            }

            // For each new center
            let mut different = false;
            for k in 0..K {
                // Finalize updating the centers of mass
                let center = new_centers[k];
                let count = new_counts[k];
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
                if algorithm == Algorithm::Simple {
                    // Get point centers
                    for i in 0..R {
                        point_centers[i] = centers.closest(&points[i]);
                    }
                }
                return KMeans { centers: centers.0, point_centers }
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
    fn fit_naive_with_random_state() {
        let points = [
            Point([0.5, 0.5]),
            Point([1.5, 0.5]),
            Point([0.5, 1.5]),
            Point([1.5, 1.5])
        ];
        let KMeans { centers, point_centers } = KMeans::<4, 2, 4>::fit_with_random_state(&points, Algorithm::Naive, 0);
        assert_eq!(centers, [
            Point([0.5, 0.5]),
            Point([1.5, 1.5]),
            Point([1.5, 0.5]),
            Point([0.5, 1.5])
        ]);
        assert_eq!(point_centers, [0, 2, 3, 1]);
    }

    #[test]
    fn fit_simple_with_random_state() {
        let points = [
            Point([0.5, 0.5]),
            Point([1.5, 0.5]),
            Point([0.5, 1.5]),
            Point([1.5, 1.5])
        ];
        let KMeans { centers, point_centers } = KMeans::<4, 2, 4>::fit_with_random_state(&points, Algorithm::Simple, 0);
        assert_eq!(centers, [
            Point([0.5, 0.5]),
            Point([1.5, 1.5]),
            Point([1.5, 0.5]),
            Point([0.5, 1.5])
        ]);
        assert_eq!(point_centers, [0, 2, 3, 1]);
    }
}
