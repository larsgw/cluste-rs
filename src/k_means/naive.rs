use crate::point::Point;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// Implements Lloyd's [k-means clustering](https://en.wikipedia.org/wiki/K-means_clustering)
/// (Lloyd, 1982), as described in (Pelleg & Moore, 1999).
///
/// # References
///
/// Lloyd, S. (1982). Least squares quantization in PCM. IEEE Transactions on Information Theory,
///     28(2), 129–137. <https://doi.org/10.1109/TIT.1982.1056489>
///
/// Pelleg, D., & Moore, A. (1999). Accelerating exact k-means algorithms with geometric reasoning.
///     Proceedings of the Fifth ACM SIGKDD International Conference
///     on Knowledge Discovery and Data Mining, 277–281. <https://doi.org/10.1145/312129.312248>
pub struct NaiveKMeans<const K: usize, const M: usize, const R: usize> {
    centers: [Point<M>; K],
    point_centers: [usize; R]
}

impl<const K: usize, const M: usize, const R: usize> NaiveKMeans<K, M, R> {
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
        let mut centers = Self::random_points(points, &mut rng);

        // Update centers
        loop {
            let mut point_centers = [0; R];
            let mut new_centers = [(); K].map(|_| (Point::<M>::default(), 0));

            // For each data point
            for i in 0..R {
                // Find the closest center
                let mut min_d = f64::INFINITY;
                let mut min_c = 0;
                for k in 0..K {
                    let d = centers[k].distance(&points[i]);
                    if d < min_d {
                        min_d = d;
                        min_c = k;
                    }
                }

                // Update the center associated with the data point
                point_centers[i] = min_c;

                // Update the center of mass
                let (center, count) = new_centers[min_c];
                new_centers[min_c] = (center + points[i], count + 1);
            }

            // For each new center
            let mut different = false;
            for k in 0..K {
                // Finalize updating the centers of mass
                let (center, count) = new_centers[k];
                let new_center = if count == 0 {
                    center
                } else {
                    center / count
                };

                // Check whether convergence is reached
                if centers[k] != new_center {
                    different = true;
                }
                centers[k] = new_center;
            }

            // If all centers are converged, return
            if !different {
                return NaiveKMeans{ centers, point_centers }
            }
        }
    }

    fn random_points(points: &[Point<M>; R], rng: &mut impl Rng) -> [Point<M>; K] {
        // Ensure initialization for compiler
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
        let NaiveKMeans { centers, point_centers } = NaiveKMeans::<4, 2, 4>::fit_with_random_state(&points, 0);
        assert_eq!(centers, [
            Point([0.5, 0.5]),
            Point([1.5, 1.5]),
            Point([1.5, 0.5]),
            Point([0.5, 1.5])
        ]);
        assert_eq!(point_centers, [0, 2, 3, 1]);
    }
}
