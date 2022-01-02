#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<const M: usize> (pub [f64; M]);

impl<const M: usize> Point<M> {
    pub fn new(coords: [f64; M]) -> Self {
        Self(coords)
    }

    pub fn default() -> Self {
        Self([0.0; M])
    }

    /// d(x, y) as defined in Section 2 (p. 278)
    ///
    /// Time complexity: O(M)
    pub fn distance(&self, point: &Self) -> f64 {
        (0..M).map(|d| (self.0[d] - point.0[d]).powi(2)).sum::<f64>().sqrt()
    }
}

impl<const M: usize> std::ops::Add for Point<M> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut coords = [0.0; M];
        for d in 0..M {
            coords[d] = self.0[d] + other.0[d];
        }
        Self(coords)
    }
}

impl<const M: usize> std::ops::Div<usize> for Point<M> {
    type Output = Self;

    fn div(self, other: usize) -> Self {
        let mut coords = [0.0; M];
        for d in 0..M {
            coords[d] = self.0[d] / (other as f64);
        }
        Self(coords)
    }
}

/// Time complexity: O(M)
pub fn get_range<const M: usize>(points: &[Point<M>]) -> (Point<M>, Point<M>) {
    let mut min = [f64::INFINITY; M];
    let mut max = [f64::NEG_INFINITY; M];

    for point in points {
        for d in 0..M {
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
