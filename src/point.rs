#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<const N: usize> (pub [f64; N]);

impl<const N: usize> Point<N> {
    pub fn new(coords: [f64; N]) -> Self {
        Point(coords)
    }

    /// d(x, y) as defined in Section 2 (p. 278)
    ///
    /// Time complexity: O(M)
    pub fn distance(&self, point: &Self) -> f64 {
        (0..N).map(|d| (self.0[d] - point.0[d]).powi(2)).sum::<f64>().sqrt()
    }
}

/// Time complexity: O(M)
pub fn get_range<const N: usize>(points: &[Point<N>]) -> (Point<N>, Point<N>) {
    let mut min = [f64::INFINITY; N];
    let mut max = [f64::NEG_INFINITY; N];

    for point in points {
        for d in 0..N {
            if point.0[d] < min[d] { min[d] = point.0[d]; }
            if point.0[d] > max[d] { max[d] = point.0[d]; }
        }
    }

    (Point(min), Point(max))
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn it_works() {
        Point([0.0, 0.2, 0.5]);
    }

    #[test]
    fn distance() {
        assert_eq!(Point::distance(
            &Point([1.0, 2.0, 3.0]),
            &Point([4.0, 5.0, 6.0])
        ), 5.196152422706632);
    }
}
