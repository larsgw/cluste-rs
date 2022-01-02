#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<const N: usize> (pub [f64; N]);

impl<const N: usize> Point<N> {
    pub fn new(coords: [f64; N]) -> Self {
        Self(coords)
    }

    pub fn default() -> Self {
        Self([0.0; N])
    }

    /// d(x, y) as defined in Section 2 (p. 278)
    ///
    /// Time complexity: O(M)
    pub fn distance(&self, point: &Self) -> f64 {
        (0..N).map(|d| (self.0[d] - point.0[d]).powi(2)).sum::<f64>().sqrt()
    }
}

impl<const N: usize> std::ops::Add for Point<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut coords = [0.0; N];
        for d in 0..N {
            coords[d] = self.0[d] + other.0[d];
        }
        Self(coords)
    }
}

impl<const N: usize> std::ops::Div<usize> for Point<N> {
    type Output = Self;

    fn div(self, other: usize) -> Self {
        let mut coords = [0.0; N];
        for d in 0..N {
            coords[d] = self.0[d] / (other as f64);
        }
        Self(coords)
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
    fn distance() {
        assert_eq!(Point::distance(
            &Point([1.0, 2.0, 3.0]),
            &Point([4.0, 5.0, 6.0])
        ), 5.196152422706632);
    }
}
